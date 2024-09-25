use std::sync::Arc;
use async_trait::async_trait;
use diesel::query_dsl::methods::FilterDsl;
use diesel::RunQueryDsl;
use diesel::OptionalExtension;
use diesel::ExpressionMethods;

use crate::presentation::handlers::user_handler::NewStudent;
use crate::domain::entities::user::Student;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::db::connection::establish_connection;
use crate::infrastructure::db::connection::DBPool;
use crate::schema;
use crate::schema::students::dsl::students;
use crate::schema::students::name;


#[derive(Clone)]
pub struct PostgresUserRepository{
    pool:DBPool
}

impl PostgresUserRepository{
    pub fn new()->Self{
        let database_url=std::env::var("DATABASE_URL").expect("DATABASE URL...");
        PostgresUserRepository{
            pool:establish_connection(&database_url)
        }
    }
}

#[async_trait]
impl UserRepository for Arc<PostgresUserRepository>{
    async fn find_by_name(&self,input_name:String)-> Option<Student>{

        students.filter(name.eq(input_name)).first::<Student>(&mut self.pool.get().unwrap())
        .optional()
        .expect("error loding user!!")

    }

    async fn save(&self,student:&NewStudent) ->Result<(),diesel::result::Error> {

        diesel::insert_into(schema::students::table)
        .values(student)
        .execute(&mut self.pool.get().unwrap())?;

        Ok(())

    }
}

