use axum::http::HeaderMap;
use axum::{
    extract::Request,
    response::Response,
};

use std::collections::HashMap;
use crate::rng;

use reqwest::Client;

use rand::random_range;
use rand::Rng;

pub async fn proxy_route(mut req: Request) -> Response {
    let client = Client::new();
    
    let params = req.uri().to_string();
    let params = params.split("?").collect::<Vec<&str>>();
    let params = params.get(1).unwrap().split("&").collect::<Vec<&str>>();
    let params = params.iter().map(|p| {
        let p = p.split("=").collect::<Vec<&str>>();
        
        (p.get(0).unwrap_or(&"").to_string(), p.get(1).unwrap_or(&"").to_string())
    }).collect::<HashMap<String, String>>();
    
    let media_id = params.get("id").unwrap();
    let def = "video".to_string();
    let media_type = params.get("media_type").unwrap_or(&def).to_lowercase();
    
    let mut rand_ident = String::new();
    
    for _ in 0..8 {
        let n: i32 = random_range(1..=10);
        rand_ident += &format!("{n}");
    }
    
    let random_string: String = (&mut rng())
            .sample_iter(&rand::distr::Alphanumeric)
            .take(22)
            .map(char::from)
            .collect();
        
    
    //
    let axum_body: axum::body::Body = std::mem::take(req.body_mut());
    let method = req.method().clone();
    let headers = req.headers();
    
    match media_type.as_str() {
        "video" => {
            let uri = format!("https://image.civitai.com/{random_string}/{media_id}/transcode=true,width=450,optimized=true/{rand_ident}");
            return video_proxy(client, axum_body, method, &uri, headers).await;
        },
        _ => {
            let uri = format!("https://image.civitai.com/{random_string}/{media_id}/anim=false,width=450,optimized=true/{rand_ident}");
            return image_proxy(client, axum_body, method, &uri, headers).await;
        }
    }
}

use axum::body::to_bytes;

async fn image_proxy(client: Client, axum_body: axum::body::Body, method: reqwest::Method, uri: &str, headers: &HeaderMap) -> Response {
    let body = to_bytes(axum_body, 1024 * 1024).await.unwrap();
    
    let mut req_builder = client
        .request(method.clone(), uri)
        .body(body);
    
    let user_token = match headers.get("user_token") {
        Some(t) => t.to_str().unwrap().to_string(),
        None => String::new()
    };
    
    req_builder = req_builder
        //.header(":authority", "image.civitai.com")
        //.header(":method", "GET")
        .header("Cookie", format!("__Secure-civitai-token={user_token}"))
        .header("cache-control", "max-age=0")
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36 OPR/124.0.0.0")
    ;
    
    let res = req_builder.send().await.unwrap();
    
    let mut builder = Response::builder().status(res.status());
    
    for (name, value) in res.headers().iter() {
        builder = builder.header(name, value);
    }
    
    let bytes = res.bytes().await.unwrap();
    
    builder.body(bytes.into()).unwrap()
}

async fn video_proxy(client: Client, axum_body: axum::body::Body, method: reqwest::Method, uri: &str, headers: &HeaderMap) -> Response {
    let stream = axum_body.into_data_stream();
    
    let reqwest_body = reqwest::Body::wrap_stream(stream);
    
    let mut req_builder = client.request(method, uri).body(reqwest_body);
    
    let user_token = match headers.get("user_token") {
        Some(t) => t.to_str().unwrap().to_string(),
        None => String::new()
    };
    
    req_builder = req_builder
        //.header(":authority", "image.civitai.com")
        //.header(":method", "GET")
        .header("Cookie", format!("__Secure-civitai-token={user_token}"))
        .header("cache-control", "max-age=0")
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36 OPR/124.0.0.0")
    ;
    
    let res = match req_builder.send().await {
        Ok(r) => r,
        Err(e) => {
            use axum::response::IntoResponse;
            println!("Error loading media: {}", e);
            return (axum::http::StatusCode::BAD_GATEWAY, format!("backend error: {}", e)).into_response();
        }
    };
    
    let mut builder = Response::builder().status(res.status());
    
    for (name, value) in res.headers().iter() {
        builder = builder.header(name, value);
    }
    
    let stream = res.bytes_stream();
    
    let body = axum::body::Body::from_stream(stream);
    
    builder.body(body).unwrap()
}