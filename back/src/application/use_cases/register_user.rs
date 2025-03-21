
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::services::user_service::UserService;
use crate::presentation::handlers::user_handler::NewUser;

pub struct RegisterUserUseCase<T: UserRepository>{
    user_service: UserService<T>
}


impl <T: UserRepository> RegisterUserUseCase<T> {
    pub fn new(user_repo: T) -> Self{
        let user_service :UserService<T> = UserService::new(user_repo);
        RegisterUserUseCase {
            user_service
        }
    }


    pub async fn execute(&self, new_user: NewUser) -> Result<(), diesel::result::Error>{
        self.user_service.register_user(new_user).await
    }

}