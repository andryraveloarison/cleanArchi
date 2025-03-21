use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::services::user_service::UserService;


pub struct GetUserUseCase<T: UserRepository> {
    user_service: UserService<T>
}

impl <T: UserRepository> GetUserUseCase<T> {
    pub fn new(user_repo: T) -> Self{
        let user_service = UserService::new(user_repo);
        GetUserUseCase {
            user_service
        }
    }

    pub async fn get(&self, email: String) -> Option<User> {
        self.user_service.get_by_email(email).await
    }
}
