mod api;

mod media_proxy;

use axum::{
    Router, http::Method
};

use rand::rng;
use tower_http::services::ServeDir;
use axum::routing::any;

use tower_http::cors::{CorsLayer, Any};
use tower::ServiceBuilder;

use tokio::{sync::oneshot};
use std::{sync::mpsc};
use notify::{Event, RecursiveMode, Result, Watcher};

pub async fn start_civit_frontend_server(port: usize, static_dir: &str) {
    // Shutdown signal
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    
    let working_dir = std::env::current_dir().unwrap();
            
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
        ;
        
    let app = Router::new()
        .nest("/civit", app)
        .fallback_service(ServeDir::new(static_dir))
        .layer(ServiceBuilder::new().layer(cors_layer)) 
    ;
    
    // Listen for file delete to shutdown server instance
    let file_to_watch = working_dir.join("civit_comfy_state");
    tokio::spawn(async move {
        
        let (tx, rx) = mpsc::channel::<Result<Event>>();
        
        let mut watcher = notify::recommended_watcher(tx).unwrap();
        
        watcher.watch(std::path::Path::new("."), RecursiveMode::Recursive).unwrap();
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
        .with_graceful_shutdown(async {
            let _ = shutdown_rx.await;
        })
        .await.unwrap()
    ;
}






