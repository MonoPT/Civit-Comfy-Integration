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

pub async fn start_civit_frontend_server(port: usize, static_dir: &str) {    
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
        .layer(ServiceBuilder::new().layer(cors_layer));
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    let addr = listener.local_addr().unwrap().port();
    println!("http://127.0.0.1:{addr}");
    
    axum::serve(listener, app).await.unwrap();
}






