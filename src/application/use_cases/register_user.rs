use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::services::user_services::UserService;
use crate::presentation::handlers::user_handler::NewStudent;

pub struct  RegisterUserUseCase<T:UserRepository>{
    user_service: UserService<T>
}

impl <T:UserRepository>RegisterUserUseCase<T>{

    pub fn new(user_repo:T)->Self{
    let user_service =UserService::new(user_repo);
    RegisterUserUseCase{
        user_service
    }
}
pub async fn execte(&self,new_user:NewStudent)->Result<(),diesel::result::Error>{
    self.user_service.register_user(new_user).await
}
}