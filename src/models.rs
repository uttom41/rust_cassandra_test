use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug,Deserialize,Serialize)]
pub struct User {
    pub id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub value1: Option<String>,
    pub value2: Option<String>,
    pub value3: Option<String>,
    pub value4: Option<String>,
    pub value5: Option<String>,
    pub value6: Option<String>,
    pub value7: Option<String>,
    pub value8: Option<String>,
    pub value9: Option<String>,
    pub value10: Option<String>,
    pub value11: Option<String>,
    pub value12: Option<String>,
    pub value13: Option<String>,
    pub value14: Option<String>,
    pub value15: Option<String>,
    pub value16: Option<String>,
    pub value17: Option<String>,
    pub value18: Option<String>,
    pub value19: Option<String>,
    pub value20: Option<String>,
    pub value21: Option<String>,
    pub value22: Option<String>,
    pub value23: Option<String>,
    pub value24: Option<String>,
    pub value25: Option<String>,
    pub value26: Option<String>,
    pub value27: Option<String>,
    pub value28: Option<String>,
    pub value29: Option<String>,
    pub value30: Option<String>,
    pub value31: Option<String>,
    pub value32: Option<String>,
    pub value33: Option<String>,
    pub value34: Option<String>,
    pub value35: Option<String>,
    pub value36: Option<String>,
    pub value37: Option<String>,
    pub value38: Option<String>,
    pub value39: Option<String>,
    pub value40: Option<String>,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct  UserRole{
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub role: Option<String>,
}