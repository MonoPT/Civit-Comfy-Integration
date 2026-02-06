mod api;
mod downloader;

mod media_proxy;

use std::sync::Arc;

use axum::{
    Router, 
    http::Method,
    Extension
};

use rand::rng;
use tower_http::services::ServeDir;
use axum::routing::any;

use tower_http::cors::{CorsLayer, Any};
use tower::ServiceBuilder;

use tokio::{sync::oneshot};
use tokio::sync::{Mutex, mpsc};
use notify::{Event, RecursiveMode, Result, Watcher};

#[derive(Clone, Debug)]
struct AppState {
    tx_downloader: mpsc::Sender<downloader::DownloadJob>,
    downloads: Arc<Mutex<Vec<ModelDownloading>>>,
    comfy_path: String
}

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
struct ModelDownloading {
    model_payload: String,
    downloaded: u64,
    total_size: u64,
    started_at: chrono::DateTime<chrono::Utc>,
    finished_at: Option<chrono::DateTime<chrono::Utc>>,
    download_speed: f64,
    status: String,
    file_name: String,
    cover: String,
    base_model: String,
    model_name: String,
    author_name: String,
    published_at: String,
    id: String,
    based_on_model: String,
    stats: String
}

static TRACKING_FILE_NAME: &str = ".civit_comfy_state";

pub async fn start_civit_frontend_server(port: usize, static_dir: &str, comfy_path: &str) {
    // Shutdown signal
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    
    // Downloader channel
    let models_being_downloaded: Arc<Mutex<Vec<ModelDownloading>>> = Arc::new(Mutex::new(vec![]));
    
    let (tx_downloader, rx_downloader) = mpsc::channel::<downloader::DownloadJob>(100);
        
    tokio::spawn(downloader::download_worker(rx_downloader, Arc::clone(&models_being_downloaded)));
    
    //
    let state = Arc::new(AppState { 
        tx_downloader,
        downloads: Arc::clone(&models_being_downloaded),
        comfy_path: comfy_path.to_string()
    });
    
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)  // Open access to selected route
        .allow_methods(vec![Method::GET, Method::POST]);
            
    let app = Router::new()
        .route("/health", any("ok"))
        .route("/media_proxy", any(media_proxy::proxy_route)) // proxy para tudo
        .merge(api::user::route())
        .merge(api::mansonary::route()) //Infinite media mansonary data
        .merge(api::tags::route())
        .merge(api::visualizer_data::route())
        .merge(api::favorite_media::route())
        .merge(api::get_collection_with_media::route())
        .merge(api::get_collections::route())
        .merge(api::update_collections::route())
        .merge(api::get_base_model_list::route())
        .merge(api::get_tools::route())
        .merge(api::get_techniques::route())
        .merge(downloader::route())
        .merge(api::get_model::route())
        .merge(api::create_collection::route())
        .merge(api::get_generation_data::route())
    ;
        
    let app = Router::new()
        .nest("/civit", app)
        .fallback_service(ServeDir::new(static_dir))
        .layer(ServiceBuilder::new().layer(cors_layer)) 
        .layer(Extension(state))
    ;
            
    // Listen for file delete to shutdown server instance    
    let working_dir = user_dirs::cache_dir().unwrap();
    let file_to_watch = working_dir.join(TRACKING_FILE_NAME);
    
    println!("{}", file_to_watch.to_string_lossy());
    
    if file_to_watch.exists() {
        let _ = std::fs::remove_file(&file_to_watch);
    }
        
    let _ = std::fs::File::create(&file_to_watch);
        
    
    tokio::spawn(async move {
        use std::{sync::mpsc};
        let (tx, rx) = mpsc::channel::<Result<Event>>();
        
        let mut watcher = notify::recommended_watcher(tx).unwrap();
        
        watcher.watch(std::path::Path::new(&working_dir), RecursiveMode::Recursive).unwrap();
        // Block forever, printing out events as they come in
        let lf = file_to_watch.to_string_lossy().replace("\\", "/").replace("//", "/"); 
        
        for res in rx {
            match res {
                Err(e) => println!("watch error: {:?}", e),
                Ok(event) => {
                    if event.kind.is_remove() {
                        for path in event.paths {
                            let path_str = path.to_string_lossy().replace("\\", "/").replace("./", "/").replace("//", "/");
                                                
                            if lf == path_str {
                                let _ = shutdown_tx.send(());
                                return;
                            }
                        }
                    }
                },
            }
        }
        
        // block forever until event triggers
        std::thread::park();
    });
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    let addr = listener.local_addr().unwrap().port();
    println!("http://127.0.0.1:{addr}");
    
    axum::serve(listener, app)
        .with_graceful_shutdown(async { // Waits for file delcared above to be removed, if so it shutdowns the isntance. prevents two instances from being running at the same time
            let _ = shutdown_rx.await;
        })
        .await.unwrap()
    ;
}






