use axum::{
    Json, Router, extract::Query, response::{IntoResponse, Response}, routing::get,
    extract::Path
};

use serde::{Deserialize};

use civit::Civit;

pub fn route() -> Router {
    Router::new()
        .route("/get_model/{model_id}", get(get_model))
}

#[derive(Deserialize)]
pub struct DataReq {
    token: String,
}

/// Gets a list of collections with media item id
pub async fn get_model(data: Query<DataReq>, Path(model_id): Path<usize>) -> Response {
    let token = &data.token;
        
    let civit = Civit::new()
        .update_api_key("api key")
        .set_auth_token(token);
    
    Json(civit.model_by_id(model_id).await).into_response()
}
