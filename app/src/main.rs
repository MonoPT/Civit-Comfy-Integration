use civit::{
    Civit,
    ModelsOptions,
    ImagesOptions,
    TagsOptions,
    DownloadOptions
};

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    
    let civit = Civit::new().update_api_key("7577577650f1dfd4e270c231f7c105de"); 
    //let response = civit.models(ModelsOptions::default().limit(1)).await;
    //let response = civit.model_by_id(1102).await;  
    //let response = civit.model_by_version(297064).await;   
    //let response = civit.tags(TagsOptions::default().limit(1).page(50)).await; 
    //let response = civit.download(297064, DownloadOptions::default()).await;
    
    //dbg!(response);
    
    let app = Router::new().route("/", get(hello_world));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3090").await.unwrap();
    let addr = listener.local_addr().unwrap().port();
    println!("http://127.0.0.1:{addr}");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}