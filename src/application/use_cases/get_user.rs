
use crate::domain::entities::user::Student;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::services::user_services::UserService;

pub struct GetUserUseCase<T:UserRepository>{
    user_service: UserService<T>
}
impl <T:UserRepository>GetUserUseCase<T>{
    pub fn new(user_repo:T)->Self{
        let user_service:UserService<T>=UserService::new(user_repo);
        GetUserUseCase{
            user_service
        }
    }

    pub async fn get(&self,name:String)->Option<Student>{
        self.user_service.get_by_name(name).await
    }
}