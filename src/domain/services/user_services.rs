use crate::presentation::handlers::user_handler::NewStudent;
use crate::domain::entities::user::Student;
use crate::domain::repositories::user_repository::UserRepository;

pub struct  UserService<T: UserRepository>{
    user_repo:T
}

impl <T:UserRepository>UserService<T>{
    pub fn new(user_repo: T)-> Self{
        UserService{
            user_repo
        }
    }

    pub async fn register_user(&self,student: NewStudent)->Result<(),diesel::result::Error>{
         self.user_repo.save(&student).await?;
         Ok(())
    }
    pub async fn get_by_name(&self ,name:String)->Option<Student>{
        self.user_repo.find_by_name(name).await
    }
}