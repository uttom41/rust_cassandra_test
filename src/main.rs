#[macro_use] extern crate rocket;

use rocket::{serde::json::Json, State};
use scylla::frame::response::result::CqlValue;
use scylla::{Session, SessionBuilder};


use uuid::Uuid;


mod  models;
use models::User;

struct CassandraDb {
    session: Session,
}

// Initialize Cassabdra Session
async fn init_db() -> Session {
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")  
        .build()
        .await
        .expect("Failed to connect to Scylla DB");

    // Create Keyspace and Table if not exists
    let result =  session.query_iter(
        "CREATE KEYSPACE IF NOT EXISTS my_saas 
        WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1}",
        &[]
    ).await;

    // Handle the result
    match result {
        Ok(_) => println!("Keyspace created successfully"),
        Err(e) => println!("Failed to create keyspace: {}", e),
    }

   // Create Table if not exists
    let result1 = session.query_iter(
        "CREATE TABLE IF NOT EXISTS my_saas.users (
            id UUID PRIMARY KEY,
            username TEXT,
            email TEXT,
            password TEXT
        )",
        &[]
    ).await;

    match result1 {
        Ok(_) => println!("Table created successfully"),
        Err(e) => println!("Failed to create table: {}", e),
    }


    session
}


#[post("/users", format = "application/json", data = "<new_user>")]
async fn create_user(new_user: Json<User>, state: &State<CassandraDb>) -> Result<String, String> {
    let id = Uuid::new_v4(); // Generate a new UUID for the user
    let insert_query = "INSERT INTO my_saas.users (id, username, email, password) VALUES (?, ?, ?, ?)";
   
    // Execute the insert query
    let result = state.session.query_iter(insert_query, (id, &new_user.username, &new_user.email, &new_user.password)).await;

    match result {
        Ok(_) => Ok(format!("User inserted successfully: {:?}", id)),
        Err(e) => Err(format!("Error inserting user: {}", e)),
    }
}

// GET API to fetch all users
#[get("/users")]
async fn get_users(state: &State<CassandraDb>) -> Json<Vec<User>> {
   
    let mut users = Vec::new();

     let rows_opt = state.session
     .query_unpaged("SELECT id, username, email, password FROM my_saas.users", &[])
     .await.expect("Data is not available");

     if let Some(rows) = rows_opt.rows {
        for row in rows {
    
            // Check if the row is Some
                // Extract values safely
                let id: Option<Uuid> = row.columns[0].as_ref().unwrap().as_uuid();
                let username: Option<String> = row.columns[1].as_ref().unwrap().as_text().cloned();
                let email: Option<String> = row.columns[2].as_ref().unwrap().as_text().cloned();
                let password: Option<String> = row.columns[3].as_ref().unwrap().as_text().cloned();
                
                let user = User {
                    id,
                    username,
                    email,
                    password,
                };
                users.push(user);
            
        }
    } else {
        eprintln!("No rows found.");
    }


   Json(users)
}

fn print_data(cqlValue : &CqlValue){
    match cqlValue {
        CqlValue::Uuid(uuid) => {
            println!("UUID: {}", uuid);
        }
        CqlValue::Int(i) => {
            println!("Integer: {}", i);
        }
        CqlValue::Text(text) => {
            println!("Text: {}", text);
        }
        _ => {
            println!("Other CqlValue type or unsupported type.");
        }
    }
}

#[launch]
 async fn rocket() -> _ {
    let session = init_db().await;
    let db = CassandraDb { session };
    rocket::build()
    .manage(db)
    .mount("/", routes![get_users,create_user])
}

