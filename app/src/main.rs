use axum::{
    Json, Router, extract::Query, http::{HeaderMap, Method, StatusCode, Uri, header}, response::{IntoResponse, Response}, routing::get
};

use serde::Deserialize;

use civit::{
    Civit, DownloadOptions, ImagesInfiniteLoadOptions, ModelsOptions, TagsOptions, UserData
};


use tower_http::cors::{CorsLayer, Any};
use tower::ServiceBuilder;


#[tokio::main]
async fn main() {
    //civit::test().await;
    /*let mut civit = Civit::new()
        .update_api_key("7577577650f1dfd4e270c231f7c105de")
        .set_auth_token("eyJhbGciOiJkaXIiLCJlbmMiOiJBMjU2R0NNIn0..2v2O5l9IiMjn_fuK.N19jcolPuTOiigD5DJ0ZyfZ1v8A_nzC0qmRexKyFTzn4mSfIURw_d_vh7yTwoIivGYi1N9-_n4x0oFLCnOteLmjfr5zniJCGLgx_VuoasNg8VX5jAwHsLhjNuyjT3P9DtFtlQueqs35t5kETy0wxwidKlRsNgHsvAp24JVgH04XCnliSmrK6_mBgnZ7VWFVucVwd2TLLvdoWXA5CiuIeEM6J2Ks2RQi_KG3lDIwjroWaMhaDxO_mrpRSV8AkDSu5jxRvO40d-jrLtfTVDwewoa6U4buNRBcTjgt6FcY_QICK_heL1enckAyd4foi65MvD93vBwE9FiHUGm3ZSxwKwsTW8XXPnkzPOrCHKpBjRlqeYmCLHJUF7igeeuLb_f-DgKosV81b7uDoWV9IH5TeYOjdHeIy-2F9BJqoAYSqHNnTE1RwPmGalRq-F8Wnb1E9uYRbnAWPFuRWYFVfvt9L-PQCWCLTCG_RkfJRbAyCFdJNfPCo2lZqF1NPSCRcBCT_G2tLJdkq91HZLWh7jj2cs5mJ3svooHJw9iXJ_-b4QCC3zeDQgOqqSEezqFEJUq4PpZknA6DMrR2sO4LDk_ZNPWzeNYk6NNHCpzuanIGbHrVlp_7wq3rb0fLs6B2r_lQMmZMPfnFxpAy9DCxQvO8jCvmUzHRPHIiTkhxwZLsnYQVjhN6FBCEgBDkYlqw0gQ6DuNDUpjK4c_9F2o4cgRXQqe313pyOcMnQuZUesNsBp4AxTGfNHohsNtAtMVjikDw2i6BD86Gpqtv_JEnYpFhGGzgMU7Kkrd_2V9YCmNFaDKcpTtZ84k8zSyYPbDNIHQTxVOTPQR9NsT5mCoxvhtPF4kg9CznZLuTU52HMQ8UlPaAcu6bbDZtepeLklhqGLhcgNK06pLJx8FgoKK3LEVZWzzIGL3VwTM48cdc1HfHpRijlXOheTKvBRGQK1kMrEcjjOeD3EOXtrWuuLU5LWZYmuUo4AXq0eP9Y44hk1cm02KGHBJbP0wSytfRQY8-UtcBM6E5GbNXfM2MkcDF0efLI1OCPGadeW4iBUF1IkWYI0Qd8Wz8tFGbswfF7BhsJNjF5K9mBQZIDcd6apjrz4i98gmD2bAYVtBQIQ6ndy4fNy30lCL_GJvex7k-_xgxhze1WFjEyjbcJdOdqq7yIwusEHJwOnBy-hqUhWw-IbujWLxogSIy7DE3K38xefehUaSKzgxyjwh3m1_Xm3gXL4oSVer3xYBwXZ88qRzP6EnEK1K6Dk2gv25GOOr1llV8dYfuwvHEbcYK-PHtgdmH3IALxB3CFa0FNfRa8tkppocQdGASB.aPgaSlCSH_XRxmP9Wl0Vyg
        "); 

    let resp = civit.load_infinite(civit::ImagesInfiniteLoadOptions::default()).await;*/
    
    
    
    
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)  // Open access to selected route
        .allow_methods(vec![Method::GET, Method::POST]);
    
    let app = Router::new()
        .route("/user_data", get(get_user))
        .route("/infinite_images", get(infinite_images))
        .layer(ServiceBuilder::new().layer(cors_layer));
    
    
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3090").await.unwrap();
    let addr = listener.local_addr().unwrap().port();
    println!("http://127.0.0.1:{addr}");
    
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct UserDataReq {
    token: String,
}

async fn get_user(data: Query<UserDataReq>) -> Response {
    let token = &data.token;
    
    let mut civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    let data = civit.user_data().await;
    
    match data {
        None => return (StatusCode::BAD_REQUEST, "Token Invalid").into_response(),
        Some(data) => {
            return (StatusCode::OK, Json(data)).into_response()
        }
    }
}

#[derive(Deserialize, Debug)]
struct InfiniteImagesDataReq {
    token: String,
    period: String,
    sort: String,
    #[serde(rename = "baseModel")]
    base_model: String,
    #[serde(rename = "mediaType")]
    media_type: String,
    cursor: String,
    #[serde(rename = "browsingLevel")]
    browsing_level: usize,
    techniques: String
}

// TODO: bas request with models gives me the base models possible

async fn infinite_images(data: Query<InfiniteImagesDataReq>) -> Response {
    let token = &data.token;
    
    let mut civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
            
    let cursor = match data.cursor.as_str() {
        "null" => None,
        _ => Some(data.cursor.clone())
    };
    
    let medias = data.media_type.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.to_lowercase().to_string()).collect::<Vec<String>>();
    let base_models = data.base_model.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.to_string()).collect::<Vec<String>>();
    let techniques = data.techniques.split(",").collect::<Vec<&str>>().iter().filter(|s| s.trim().len() > 0).map(|f| f.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    
    // Options
    let mut options = ImagesInfiniteLoadOptions::default();
    options.cursor = cursor;
    options.period = data.period.clone();
    options.types = medias;
    options.sort = data.sort.clone();
    options.browsingLevel = data.browsing_level;
    options.base_models = base_models;
    options.techniques = techniques;
    
    //
    let data = civit.load_infinite(options).await;
    
    match data {
        None => return (StatusCode::INTERNAL_SERVER_ERROR, "500 Internal server error").into_response(),
        Some(data) => {
            return (StatusCode::OK, Json(data)).into_response()
        }
    }
}

