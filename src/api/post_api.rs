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

#[get("/post/{id}")]
pub async fn get_post(db: Data<AppState>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(Error{error: "Invalid Id".to_string()});
    }
    let user_detail = db.post_repo.get_post(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::BadRequest().json(Error{error: err.to_string()})
    }
}

#[put("/post/{id}")]
pub async fn update_post(db: Data<AppState>, path: Path<String>, new_post: Json<Post>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(Error{error: "Invalid id".to_owned()})
    }
    let post_id = ObjectId::parse_str(&id);
    let post_id = match post_id {
        Ok(obj_id) => obj_id,
        Err(e) => return HttpResponse::BadRequest().json(Error{error: e.to_string()})
    };
    let post = db.post_repo.get_post(&id).await;
    match post {
        Err(_) => return HttpResponse::NotFound().json(Error{error: "Post not found".to_string()}),
        _ => ()
    }
    let data = Post {
        id: Some(post_id),
        title: new_post.title.to_owned(),
        content: new_post.content.to_owned(),
    };
    let update_result = db.post_repo.update_post(&id, data).await;
    match update_result {
        Ok(_) => {
            let post = db.post_repo.get_post(&id).await.ok();
            HttpResponse::Ok().json(Some(post))
        },
        Err(err) => HttpResponse::BadRequest().json(Error{error: err.to_string()})
    }
}

#[delete("/post/{id}")]
pub async fn delete_post(db: Data<AppState>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(Error{error: "Invalid ID".to_string()});
    };
    let result = db.post_repo.remove_post(&id, &db.user_repo).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json(Error{error: "Post successfully deleted!".to_string()});
            } else {
                return HttpResponse::NotFound().json(Error{error: "Post with specified ID not found".to_string()});
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}