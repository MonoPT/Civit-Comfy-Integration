mod api;
mod filters;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use crate::api::{Civit, ImagesOptions, ModelsOptions, TagsOptions, DownloadOptions};
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;
use std::io::Write;


#[tokio::main]
async fn main() {
    let civit = Civit::new("7577577650f1dfd4e270c231f7c105de"); 
    
    /* 
    let client = reqwest::Client::new();
    
    let response = client
            .get(format!("https://civitai.com/api/download/models/29109"))
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer 7577577650f1dfd4e270c231f7c105de"))
            .send().await.unwrap();
    
    let total_size = response
          .content_length()
          .ok_or("Sem content-length no header").unwrap();
    
    println!("Tamanho total: {} bytes", total_size);
    
    let mut file = tokio::fs::File::create("download.bin").await.unwrap();
    let mut downloaded: u64 = 0;

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();

        file.write_all(&chunk).await.unwrap();
        downloaded += chunk.len() as u64;

        let percent = (downloaded as f64 / total_size as f64) * 100.0;
        print!("\rProgresso: {:.2}%", percent);
        std::io::stdout().flush().ok();
    }

    println!("\nDownload concluído!");
    */
    
    
    //dbg!(response);
    
    //let response = civit.models(ModelsOptions::default().limit(1)).await;
    //let response = civit.model_by_id(1102).await;  
    //let response = civit.model_by_version(297064).await;   
    //let response = civit.tags(TagsOptions::default().limit(1).page(50)).await; 
    civit.download(297064, DownloadOptions::default()).await;
    
    //println!("{response:#?}"); 
}
