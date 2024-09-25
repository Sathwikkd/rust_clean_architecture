use diesel::Queryable;
use serde::Serialize;


#[derive(Debug,Clone,Serialize,Queryable)]
pub struct Student{
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String
}