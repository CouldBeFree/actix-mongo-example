use crate::{models::user_model::User, app_state::app_state::AppState};
use actix_web::{
    post,
    get,
    put,
    delete,
    web::{Data, Json, Path},
    HttpResponse,
};

use mongodb::bson::oid::ObjectId;

#[post("/user")]
pub async fn create_user(db: Data<AppState>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        posts: new_user.posts.to_owned()
    };
    let user_detail = db.user_repo.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            println!("Error");
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<AppState>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid Id");
    }
    let user_detail = db.user_repo.get_user(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[put("/user/{id}")]
pub async fn update_user(db: Data<AppState>, path: Path<String>, new_user: Json<User>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        posts: new_user.posts.to_owned()
    };
    let update_result = db.user_repo.update_user(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let update_user_info = db.user_repo.get_user(&id).await;
                return match update_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string())
                };
            } else {
                return HttpResponse::NotFound().body("No user found with given ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[delete("/user/{id}")]
pub async fn delete_user(db: Data<AppState>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    };
    let result = db.user_repo.delete_user(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("User successfully deleted!")
            } else {
                return HttpResponse::NotFound().json("User with specified ID not found");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn get_all_users(db: Data<AppState>) -> HttpResponse {
    let users = db.user_repo.get_all_users().await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}
