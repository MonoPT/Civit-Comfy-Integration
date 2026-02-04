use std::{error::Error, sync::Arc};

use super::{DownloadJob};

use tokio::sync::mpsc::Receiver;

use tokio::sync::{Mutex, mpsc};
use uuid::Uuid;

use crate::{ModelDownloading};

use std::sync::{
    atomic::{AtomicU64, Ordering},
};
use futures::StreamExt;

#[derive(Debug)]
pub struct ModelStatusMessage {
    model_payload: String,
    downloaded: u64,
    total: u64,
    download_speed: f64,
    status: String,
    file_name: String
}
  
use tokio::{
    fs::OpenOptions,
};

pub async fn download_worker(mut rx_downloader: Receiver<DownloadJob>, models_being_downloaded: Arc<Mutex<Vec<ModelDownloading>>>) {
    let (tx_task, mut rx_task) = mpsc::channel::<ModelStatusMessage>(100);
    
    let downloading_models = models_being_downloaded.clone();
    
    // task that receives messages from all spawned jobs
    tokio::spawn(async move {
        let downloading_models = downloading_models.clone();
        
        while let Some(msg) = rx_task.recv().await {
            let downloading_models = downloading_models.clone();
            
            let mut model = downloading_models.lock().await;
            
            let model = model.iter_mut().find(|model| model.model_payload == msg.model_payload);
                        
            match model {
                None => (),
                Some(model) => {
                    model.downloaded = msg.downloaded;
                    model.total_size = msg.total;
                    model.download_speed = msg.download_speed;
                    model.status = msg.status;
                    
                    if model.file_name.len() < 1 {
                        model.file_name = msg.file_name;
                    }
                    
                    if msg.downloaded >= msg.total {
                        model.finished_at = Some(chrono::Utc::now());
                    }
                }            
            }
        }
    });
    
    while let Some(job) = rx_downloader.recv().await {
        {
            let model = models_being_downloaded.clone();
            
            let model = model.lock().await;
            
            let model = model.iter().find(|model| {
                model.model_payload == job.payload
            });
            
            match model {
                None => (),
                Some(model) => {                  
                    if model.status == "downloading" {
                        println!("Model {} already being downloading. Skiping request", &job.payload);
                        continue;
                    }
                    
                    println!("Model {} already being downloading. Skiping request", &job.payload);
                    continue;
                }
            }

        }
        
        models_being_downloaded.clone().lock().await.push(ModelDownloading {
            id: Uuid::new_v4().to_string(),
            model_payload: job.payload.clone(),
            downloaded: 0,
            total_size: 0,
            started_at: chrono::Utc::now(),
            finished_at: None,
            download_speed: 0.0,
            status: String::from("downloading"),
            file_name: job.file_name.clone(),
            cover: job.cover.clone(),
            base_model: job.base_model.clone(),
            model_name: job.model_name.clone(),
            author_name: job.author_name.clone(),
            published_at: job.published_at.clone(),
            based_on_model: job.based_on_model.clone(),
            stats: job.stats.clone()
        });
        
        let tx_task = tx_task.clone();
        
        tokio::spawn(async move {
            process_job(job, tx_task).await;
        });
    }
}

async fn process_job(
    job: DownloadJob,
    tx: mpsc::Sender<ModelStatusMessage>,
) {
    let payload_name = job.payload.clone();
   
    let models_dir = &job.models_dir;
    
    let _ = tokio::fs::create_dir_all(&models_dir).await;
    
    println!("Models dir {:?}", models_dir);
    
    if !models_dir.is_dir() {
        println!("Invalid download path");
        return;
    }
    
    let target_folder = models_dir.join(&job.model_type);
    
    let _ = tokio::fs::create_dir_all(&target_folder).await;
    
    if !std::path::Path::new(&target_folder).is_dir() {
        

        println!("Could not create target folder");
        return;
    }
    
    
    const CONNECTIONS: u64 = 8;
    let download_url = format!("https://civitai.com/api/download/models/{}", job.payload);
    
    println!("{download_url}");
    
    let token_value = job.user_token;
    
    let cookie_value = format!("__Secure-civitai-token={token_value}");
        
    let cookie = HeaderValue::from_str(&cookie_value).unwrap();
    
    let client = reqwest::Client::builder()
        .gzip(false)
        .brotli(false)
        .deflate(false)
        .build().unwrap();
            
    let (size, file_name) = get_size(&client, &download_url, &cookie).await.unwrap();
            
    let file_n_o = &file_name;
    
    let mut file_name = std::path::Path::new(&target_folder).join(file_n_o);
    
    if file_name.is_file() { // Creates file with diff name if model already exists
        let f_name = format!("{}_{}", chrono::Utc::now().timestamp(), file_n_o);        
        file_name = std::path::Path::new(&target_folder).join(f_name);
    }
 
    
    let file = Arc::new(Mutex::new(
        match OpenOptions::new()
            .create(true)
            .write(true)
            .open(&file_name)
            .await {
                Ok(f) => f,
                Err(e) => {
                    println!("{e}");
                    let _ = tx.send(ModelStatusMessage {
                        model_payload: payload_name,
                        downloaded: 0,
                        total: size,
                        download_speed: 0.0,
                        status: "error".to_string(),
                        file_name: String::new()
                    }).await;
                    return;
                }
            }
    ));
    
    {
        let f = file.lock().await;
        f.set_len(size).await.unwrap();
    }
    
    let chunk_size = std::cmp::max(1, size / CONNECTIONS);
        
    let downloaded = Arc::new(AtomicU64::new(0));
        
    let handles = download_loop(
        &client,
        tx.clone(),
        CONNECTIONS,
        payload_name.clone(),
        download_url.clone(),
        &cookie,
        Arc::clone(&file),
        Arc::clone(&downloaded),
        size,
        chunk_size,
        &file_n_o
    ).await;
        
    let mut status = String::from("finished");
    let mut total_downloaded = size;
        
    match handles {
        Err(_e) => {            
            status = String::from("error"); 
            total_downloaded = downloaded.fetch_add(0, Ordering::Relaxed);
        },
        Ok(handles) => {            
            for h in handles {
                h.await.unwrap().unwrap();
            }
        }
    }
    
    let _ = tx.send(ModelStatusMessage {
        model_payload: payload_name,
        downloaded: total_downloaded,
        total: size,
        download_speed: 0.0,
        status: status,
        file_name: String::new()
    }).await;
}

use tokio::fs::File;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tokio::task::JoinHandle;

async fn download_loop(
    client: &reqwest::Client,
    tx: mpsc::Sender<ModelStatusMessage>,
    connections: u64,
    payload_name: String,
    download_url: String,
    cookie: &HeaderValue,
    file: Arc<Mutex<File>>,
    downloaded: Arc<AtomicU64>,
    size: u64,
    chunk_size: u64,
    file_name: &str
) -> Result<Vec<JoinHandle<Result<(), Box<dyn Error + Send + Sync>>>>, ()> {
    let mut handles = Vec::new();
    
    for i in 0..connections {
        let file_name = file_name.to_string();
        let tx = tx.clone();
        let payload_name = payload_name.clone();
        
        let download_url = download_url.clone();
        let cookie = cookie.clone();
        
        let client = client.clone();
        let file = Arc::clone(&file);
        let downloaded = Arc::clone(&downloaded);
        
        let start = i * chunk_size;
        let end = if i == connections - 1 {
            size - 1
        } else {
            (i + 1) * chunk_size - 1
        };

        let handle = tokio::spawn(async move {
            let tx = tx.clone();
            
            let resp = client
                .get(download_url)
                .header(COOKIE, cookie.clone())
                .header(reqwest::header::RANGE, format!("bytes={}-{}", start, end))
                .send()
                .await?;

            let mut stream = resp.bytes_stream();
            let mut offset = start;
 
            let start = std::time::Instant::now();
            
            while let Some(chunk) = stream.next().await {
                let file_name = file_name.clone();
                let chunk = chunk?;
                let payload_name = payload_name.clone();
                
                {
                    let mut file = file.lock().await;
                    file.seek(std::io::SeekFrom::Start(offset)).await?;
                    file.write_all(&chunk).await?;
                }
    
                offset += chunk.len() as u64;
    
                let current = downloaded.fetch_add(chunk.len() as u64, Ordering::Relaxed)
                    + chunk.len() as u64;
                
                // Calculate download speed
                let elapsed = start.elapsed().as_secs_f64(); // seconds as f64
                let download_speed = current as f64 / elapsed; // bytes per second
                
                let _ = tx.send(ModelStatusMessage {
                    model_payload: payload_name,
                    downloaded: current,
                    total: size,
                    download_speed: download_speed,
                    status: String::from("downloading"),
                    file_name: file_name
                }).await;
            }
            
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });

        handles.push(handle);
    }
    
    Ok(handles)
}

use reqwest::header::{CONTENT_RANGE, ACCEPT_ENCODING, CONTENT_DISPOSITION};
use reqwest::header::{COOKIE, HeaderValue};
/// Gets the file total size
async fn get_size(
    client: &reqwest::Client,
    url: &str,
    token_cookie: &HeaderValue
) -> Result<(u64, String), Box<dyn Error + Send + Sync>> {    
    // Fallback: ranged GET
    let resp = client
        .get(url)
        .header(reqwest::header::RANGE, "bytes=0-0")
        .header(ACCEPT_ENCODING, "identity")
        .header(COOKIE, token_cookie)
        .send()
        .await?;
    
    if resp.status() != reqwest::StatusCode::PARTIAL_CONTENT {
        return Err("Server does not support byte ranges".into());
    }

    let range = resp
        .headers()
        .get(CONTENT_RANGE)
        .ok_or("Missing Content-Range")?
        .to_str()?;

    let content_disp = resp.headers().get(CONTENT_DISPOSITION).ok_or("TODO_Random_String.model")?
        .to_str()?;
    
    let file_name = content_disp
        .rsplit('\\')
        .next()
        .ok_or("Invalid File Name")?
        .parse::<String>()?
        .split("filename=\"").collect::<Vec<&str>>().iter_mut().map(|s| s.to_string()).collect::<Vec<String>>();
    
    let file_name = match file_name.get(1) {
        Some(f) => f.to_string().replace("\"", ""),
        None => String::from("Invalid_file_name")
    };
    
      
    // bytes 0-0/6442450944
    let total = range
        .rsplit('/')
        .next()
        .ok_or("Invalid Content-Range")?
        .parse::<u64>()?;

    Ok((total, file_name))
}