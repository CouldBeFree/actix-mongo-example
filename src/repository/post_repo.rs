extern crate dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::InsertOneResult,
    Collection, Database
};

use crate::models::post_model::Post;

use super::user_repo::UserRepo;

#[derive(Clone, Debug)]
pub struct PostRepo {
    pub col: Collection<Post>
}

impl PostRepo {
    pub async fn init(db: &Database) -> Self {
        let col: Collection<Post> = db.collection("post");
        PostRepo { col }
    }

    pub async fn create_post(&self, new_post: Post, user_repo: &UserRepo) -> Result<InsertOneResult, Error> {
        let user_id = "65254ced6813e0fc1019cff9".to_string();
        let user = user_repo.get_user(&user_id).await;
        match user {
            Ok(user) => {
                let new_doc = Post {
                    id: None,
                    title: new_post.title,
                    content: new_post.content,
                };
                let post = self
                    .col
                    .insert_one(new_doc, None)
                    .await
                    .ok()
                    .expect("Error creating user");
                let id = &post.inserted_id;
                let new_doc = doc! {
                    "$push": {
                        "posts": id
                    }
                };
                user_repo
                    .col
                    .update_one(doc! {"_id": user.id}, new_doc, None)
                    .await
                    .ok()
                    .expect("Error updating user");
                return Ok(post)
            },
            Err(e) => return Err(Error::DeserializationError{message: e.to_string()}),
        }
    }
}
