use crate::{models::post_model::Post, app_state::app_state::AppState};
use actix_web::{
    post,
    get,
    put,
    delete,
    web::{Data, Json, Path},
    HttpResponse,
};
use serde::Serialize;
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize)]
struct Error {
    error: String,
}

#[post("/post")]
pub async fn create_post(db: Data<AppState>, new_post: Json<Post>) -> HttpResponse {
    let post = Post {
        id: None,
        title: new_post.title.to_owned(),
        content: new_post.content.to_owned()
    };
    let post_detail = db.post_repo.create_post(post, &db.user_repo).await;
    match post_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            HttpResponse::BadRequest().json(Error{error: err.to_string()})
        }
    }
}