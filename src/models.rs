use scylla::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug,Deserialize,Serialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,

}

#[derive(FromRow)]
pub struct  UserData{

}