use crate::PostRepo;
use crate::UserRepo;

#[derive(Clone, Debug)]
pub struct AppState {
    pub user_repo: UserRepo,
    pub post_repo: PostRepo
}