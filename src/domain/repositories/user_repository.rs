use async_trait::async_trait;


use crate::presentation::handlers::user_handler::NewStudent;
use crate::domain::entities::user::Student;



#[async_trait]
pub trait UserRepository{
    async fn find_by_name(&self,name:String) ->Option<Student>;

    async fn save(&self,student:&NewStudent) ->Result<(),diesel::result::Error>;
}