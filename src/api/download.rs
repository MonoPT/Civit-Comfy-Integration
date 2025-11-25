use std::collections::HashMap;
use std::time::Instant;
use super::Civit;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;
use std::io::Write;

#[derive(Debug, Default)]
pub struct DownloadOptions {
    model_type: String, 
    format: String, 
    size: String, 
    fp: String
}

impl DownloadOptions {
    pub fn model_type(mut self, model_type: impl ToString) -> Self {
        self.model_type = model_type.to_string();
        
        self
    }
    
    pub fn format(mut self, format: impl ToString) -> Self {
        self.format = format.to_string();
        
        self
    }
    
    pub fn size(mut self, size: impl ToString) -> Self {
        self.size = size.to_string();
        
        self
    }
    
    pub fn fp(mut self, fp: impl ToString) -> Self {
        self.fp = fp.to_string();
        
        self
    }
}

impl Civit {
    pub async fn download(&self, model_id: usize, download_options: DownloadOptions) {
        let client = &self.client;
              
        let mut params: HashMap<String, String> = HashMap::new();
        
        if download_options.model_type.len() > 0 {
            params.insert(String::from("type"), download_options.model_type.to_string());
        }
        
        if download_options.format.len() > 0 {
            params.insert(String::from("format"), download_options.format.to_string());
        }
        
        if download_options.size.len() > 0 {
            params.insert(String::from("size"), download_options.size.to_string());
        }
        
        if download_options.fp.len() > 0 {
            params.insert(String::from("fp"), download_options.fp.to_string());
        }
        
        let p = params.iter()
            .map(|p| format!("{}={}", p.0.trim(), p.1.trim())).collect::<Vec<String>>()
            .join("&");
        
        
        let response = client
                .get(format!("https://civitai.com/api/download/models/{model_id}?{p}"))
                .header(CONTENT_TYPE, "application/json")
                .header(AUTHORIZATION, format!("Bearer 7577577650f1dfd4e270c231f7c105de"))
                .send().await.unwrap();
        
        let total_size = response
              .content_length()
              .ok_or("Sem content-length no header").unwrap();
        
        println!("Tamanho total: {} bytes", total_size);
        
        let mut file = tokio::fs::File::create("model.temp").await.unwrap();
        let mut downloaded: u64 = 0;
    
        let mut stream = response.bytes_stream();
        let start_time = Instant::now();
    
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.unwrap();
    
            file.write_all(&chunk).await.unwrap();
            downloaded += chunk.len() as u64;
            
            let elapsed = start_time.elapsed().as_secs_f64(); // time in seconds
            let speed = downloaded as f64 / elapsed; // bytes per second
            let percent = (downloaded as f64 / total_size as f64) * 100.0;
            //print!("\rProgresso: {:.2}%", percent);
            
            print!(
                "\rProgresso: {:.2}% - Velocidade: {:.2} KB/s",
                percent,
                speed / 1024.0
            );
            
            std::io::stdout().flush().ok();
        }
    
        println!("\nDownload concluído!");
        
        todo!()
    }
}