
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::{get,post};
use diesel::prelude::Insertable;
use serde::Deserialize;
use log::error;

use crate::application::use_cases::register_user::RegisterUserUseCase;
use crate::application::use_cases::get_user::GetUserUseCase;
use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository;
use crate::schema::students;

#[derive(Debug,Clone,Deserialize,Insertable)]
#[diesel(table_name = students)]
pub struct NewStudent{
    pub name: String,
    pub email: String,
    pub password: String
}

#[post("/")]
pub async fn register_user_handler(
    repo: web::Data<PostgresUserRepository>,
    input: web::Json<NewStudent>
)->HttpResponse {

    match RegisterUserUseCase::new(repo.into_inner())
    .execte(input.into_inner()).await {

        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            error!("error registering student!! {:?}",err);
            HttpResponse::InternalServerError().body("please try again!")
        }
    }
    
}

#[get("/{name}")]
pub async fn get_by_name(
    repo: web::Data<PostgresUserRepository>,
    path: web::Path<(String,)>
)->HttpResponse{

    match GetUserUseCase::new(repo.into_inner()).get(path.into_inner().0).await {
        Some(student) => HttpResponse::Ok().json(student),
        None => HttpResponse::NotFound().body("student not found")
    }
}