use axum::{
    Json, Router, extract::Query, response::{IntoResponse, Response}, routing::get
};

use serde::{Deserialize};

use civit::Civit;

pub fn route() -> Router {
    Router::new()
        .route("/get_techniques_list", get(get_techniques))
}

#[derive(Deserialize)]
pub struct DataReq {
    token: String,
}

/// Gets a list of collections with media item id
pub async fn get_techniques(data: Query<DataReq>) -> Response {
    let token = &data.token;
        
    let civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    Json(civit.get_techniques_list().await).into_response()
}
