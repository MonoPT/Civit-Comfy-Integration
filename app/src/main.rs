mod api;

use axum::{
    Router, http::Method
};

use tower_http::services::ServeDir;



use tower_http::cors::{CorsLayer, Any};
use tower::ServiceBuilder;

// TODO: bas request with models gives me the base models possible

#[tokio::main]
async fn main() {
    
    let static_files = format!("{}/front-end/build", std::env::current_dir().unwrap().to_string_lossy());
    println!("Static folder: {static_files}");
    
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)  // Open access to selected route
        .allow_methods(vec![Method::GET, Method::POST]);
    
    let app = Router::new()
        .merge(api::user::route())
        .merge(api::mansonary::route()) //Infinite media mansonary data
        .merge(api::tags::route())
        
        .fallback_service(ServeDir::new(static_files))
        .layer(ServiceBuilder::new().layer(cors_layer));
    
    
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3090").await.unwrap();
    let addr = listener.local_addr().unwrap().port();
    println!("http://127.0.0.1:{addr}");
    
    axum::serve(listener, app).await.unwrap();
}




