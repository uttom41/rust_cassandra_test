#[macro_use] extern crate rocket;

use prometheus::core::Collector;
use rocket::{serde::json::Json, State,tokio::sync::Mutex};
use scylla::serialize::value::SerializeValue;
use scylla::{Session, SessionBuilder};

use uuid::Uuid;
use prometheus::{IntGauge, Registry, Encoder, TextEncoder};
use prometheus::proto::MetricFamily;
use sysinfo::System;
mod  models;
use models::{User, UserRole};
use std::sync::Arc;



struct Metrics {
    cpu_usage: IntGauge,
    ram_usage: IntGauge,
}

impl Metrics {
    fn new(registry: &Registry) -> Metrics {
        let cpu_usage = IntGauge::new("rust_cpu_usage", "CPU usage in percent").unwrap();
        let ram_usage = IntGauge::new("rust_ram_usage", "RAM usage in percent").unwrap();

        // Register the metrics with the given registry
        registry.register(Box::new(cpu_usage.clone())).unwrap();
        registry.register(Box::new(ram_usage.clone())).unwrap();

        Metrics { cpu_usage, ram_usage }
    }
}

#[rocket::get("/metrics")]
async fn metrics_handler(metrics: &State<Arc<Mutex<Metrics>>>) -> String {
    let mut sys = System::new_all(); // Create a new System instance
    sys.refresh_cpu_all(); // Refresh all CPU data
    sys.refresh_memory(); // Refresh memory data


    let mut ram_usage_kb= 0;

     // Get the current process (your project's process)
     if let Some(process) = sys.process(sysinfo::get_current_pid().expect("Unable to get current PID")) {
        // RAM usage in KB
         ram_usage_kb = process.memory();
    }


    // Get the overall CPU usage as a percentage
    let cpu_usage: f32 = sys.global_cpu_usage(); // Get overall CPU usage

    // Calculate RAM usage as a percentage
   // let total_memory = sys.total_memory();
    //let used_memory = sys.used_memory();
    let ram_usage = ram_usage_kb as i64;
    //let ram_usage = (used_memory * 100 / total_memory) as i64;

    // Lock the metrics for thread safety
    let mut metrics_guard = metrics.lock().await;
    metrics_guard.cpu_usage.set(cpu_usage as i64); // Set CPU usage in the gauge
    metrics_guard.ram_usage.set(ram_usage); // Set RAM usage in the gauge

    // Encode the metrics for Prometheus
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();

    // Collect metrics into a vector
    let metric_families: Vec<MetricFamily> = vec![
        metrics_guard.cpu_usage.collect(),
        metrics_guard.ram_usage.collect(),
    ].into_iter().flat_map(|mf| mf).collect();

    // Encode the metrics
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap() // Convert buffer to String
}


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
            id TEXT PRIMARY KEY,
            username TEXT,
            email TEXT,
            password TEXT,
            value1 TEXT,
            value2 TEXT,
            value3 TEXT,
            value4 TEXT,
            value5 TEXT,
            value6 TEXT,
            value7 TEXT,
            value8 TEXT,
            value9 TEXT,
            value10 TEXT,
            value11 TEXT,
            value12 TEXT,
            value13 TEXT,
            value14 TEXT,
            value15 TEXT,
            value16 TEXT,
            value17 TEXT,
            value18 TEXT,
            value19 TEXT,
            value20 TEXT,
            value21 TEXT,
            value22 TEXT,
            value23 TEXT,
            value24 TEXT,
            value25 TEXT,
            value26 TEXT,
            value27 TEXT,
            value28 TEXT,
            value29 TEXT,
            value30 TEXT,
            value31 TEXT,
            value32 TEXT,
            value33 TEXT,
            value34 TEXT,
            value35 TEXT,
            value36 TEXT,
            value37 TEXT,
            value38 TEXT,
            value39 TEXT,
            value40 TEXT
        )",
        &[]
    ).await;


    let result2 = session.query_iter(
        "CREATE TABLE IF NOT EXISTS my_saas.user_role (
            id TEXT PRIMARY KEY,
            user_id TEXT,
            role TEXT
        )",
        &[]
    ).await;

    match result1 {
        Ok(_) => println!("Table created successfully"),
        Err(e) => println!("Failed to create table: {}", e),
    }

    match result2 {
        Ok(_) => println!("Table created successfully"),
        Err(e) => println!("Failed to create table: {}", e),
    }


    session
}


#[post("/users", format = "application/json", data = "<new_user>")]
async fn create_user(new_user: Json<User>, state: &State<CassandraDb>) -> Result<String, String> {
    let id = Uuid::new_v4(); // Generate a new UUID for the user
    let insert_query = "INSERT INTO my_saas.users (
    id, username, email, password,
    value1, value2, value3, value4, value5, value6, value7, value8, value9, value10,
     value11, value12, value13, value14, value15, value16, value17, value18, value19, value20,
     value21, value22, value23, value24, value25, value26, value27, value28, value29, value30,
      value31, value32, value33, value34, value35, value36, value37, value38, value39, value40
    ) VALUES (
     ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
     ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 
     ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 
     ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 
     ?, ?, ?, ?)";
   
     let result = state.session.query_unpaged(insert_query, [
        id.to_string(),
        new_user.username.clone().unwrap_or_default(),
        new_user.email.clone().unwrap_or_default(),
        new_user.password.clone().unwrap_or_default(),
        new_user.value1.clone().unwrap_or_default(),
        new_user.value2.clone().unwrap_or_default(),
        new_user.value3.clone().unwrap_or_default(),
        new_user.value4.clone().unwrap_or_default(),
        new_user.value5.clone().unwrap_or_default(),
        new_user.value6.clone().unwrap_or_default(),
        new_user.value7.clone().unwrap_or_default(),
        new_user.value8.clone().unwrap_or_default(),
        new_user.value9.clone().unwrap_or_default(),
        new_user.value10.clone().unwrap_or_default(),
        new_user.value11.clone().unwrap_or_default(),
        new_user.value12.clone().unwrap_or_default(),
        new_user.value13.clone().unwrap_or_default(),
        new_user.value14.clone().unwrap_or_default(),
        new_user.value15.clone().unwrap_or_default(),
        new_user.value16.clone().unwrap_or_default(),
        new_user.value17.clone().unwrap_or_default(),
        new_user.value18.clone().unwrap_or_default(),
        new_user.value19.clone().unwrap_or_default(),
        new_user.value20.clone().unwrap_or_default(),
        new_user.value21.clone().unwrap_or_default(),
        new_user.value22.clone().unwrap_or_default(),
        new_user.value23.clone().unwrap_or_default(),
        new_user.value24.clone().unwrap_or_default(),
        new_user.value25.clone().unwrap_or_default(),
        new_user.value26.clone().unwrap_or_default(),
        new_user.value27.clone().unwrap_or_default(),
        new_user.value28.clone().unwrap_or_default(),
        new_user.value29.clone().unwrap_or_default(),
        new_user.value30.clone().unwrap_or_default(),
        new_user.value31.clone().unwrap_or_default(),
        new_user.value32.clone().unwrap_or_default(),
        new_user.value33.clone().unwrap_or_default(),
        new_user.value34.clone().unwrap_or_default(),
        new_user.value35.clone().unwrap_or_default(),
        new_user.value36.clone().unwrap_or_default(),
        new_user.value37.clone().unwrap_or_default(),
        new_user.value38.clone().unwrap_or_default(),
        new_user.value39.clone().unwrap_or_default(),
        new_user.value40.clone().unwrap_or_default(),
    ].as_ref()).await;
    
    let user_id = id.clone().to_string();

    match result {
        Ok(_) =>{
            let role = insert_role(&user_id,state).await;
             Ok( format!("User inserted successfully: {:?},{:?}", id,role))
             }
        Err(e) => Err(format!("Error inserting user: {}", e)),
    }
}

async fn insert_role(user_id: &String, state: &State<CassandraDb>) ->Json<UserRole> {
    let id = Uuid::new_v4(); 
    let insert_query = "INSERT INTO my_saas.user_role (id, user_id, role) VALUES (?,?,?)";

   let result = state.session.query_unpaged(insert_query, [
        id.to_string(),
        user_id.clone(),
        "admin".to_string(),
    ].as_ref()).await.expect("Data is not available");

    let role = UserRole{
        id: id.to_string(),
        user_id: user_id.clone(),
        role: Some("admin".to_string()),
    };
    Json(role)

}


#[get("/insert?<count>")]
async fn get_users_insert(count: Option<i32>, state: &State<CassandraDb>) -> Result<String, String> {
    let count = count.unwrap_or(10); // Default to 10 if count is not provided
    
    for i in 0..count {
        let id = Uuid::new_v4(); // Generate a new UUID for the user
        let insert_query = "INSERT INTO my_saas.users (
        id, username, email, password,
        value1, value2, value3, value4, value5, value6, value7, value8, value9, value10,
         value11, value12, value13, value14, value15, value16, value17, value18, value19, value20,
         value21, value22, value23, value24, value25, value26, value27, value28, value29, value30,
          value31, value32, value33, value34, value35, value36, value37, value38, value39, value40
        ) VALUES (
         ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
         ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 
         ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 
         ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 
         ?, ?, ?, ?)";

         // Collect all values dynamically
    let mut values: Vec<String> = Vec::with_capacity(44);
    
    // Add the mandatory fields
    values.push(id.to_string());
    values.push(format!("user_name_{}", i));
    values.push(format!("user_email_{}@example.com", i));
    values.push("password".to_string());

    // Generate dynamic values from value1 to value40
    for j in 1..=40 {
        values.push(format!("value{}_{}", j, i));
    }

// Convert `values` to references of `&String`
    let query_values: Vec<&String> = values.iter().collect();

    let result = state.session
    .query_unpaged(insert_query, query_values.as_slice())
    .await
    .expect("Data is not available");

    }

    
    Ok("user".to_string()) // Return users as JSON
}

// GET API to fetch all users
#[get("/users")]
async fn get_users(state: &State<CassandraDb>) -> Json<Vec<User>> {
   
    let mut users = Vec::new();

    if let Some(rows) = state.session
        .query_unpaged(
            "SELECT id, username, email, password, value1, value2, value3, value4, value5, value6, value7, value8, value9, value10,
             value11, value12, value13, value14, value15, value16, value17, value18, value19, value20,
             value21, value22, value23, value24, value25, value26, value27, value28, value29, value30,
             value31, value32, value33, value34, value35, value36, value37, value38, value39, value40
             FROM my_saas.users", &[]
        )
        .await
        .expect("Data is not available")
        .rows
    {
        for row in rows {
            // Extract the ID safely, if None, skip this row
            let id: Option<String> = row.columns[0].as_ref().and_then(|col| col.as_text().cloned());
            let id = match id {
                Some(id) => id,
                None => continue,
            };
    
            // Extract all values in a loop for brevity
            let mut values = vec![];
            for col in row.columns.iter().skip(1).take(40) {
                values.push(col.as_ref().and_then(|c| c.as_text().cloned()));
            }
    
            // Create user and push to users vector
            let user = User {
                id,
                username: values[0].clone(),
                email: values[1].clone(),
                password: values[2].clone(),
                value1: values[3].clone(),
                value2: values[4].clone(),
                value3: values[4].clone(),
                value4: values[4].clone(),
                value5: values[4].clone(),
                value6: values[4].clone(),
                value7: values[4].clone(),
                value8: values[4].clone(),
                value9: values[4].clone(),
                value10: values[4].clone(),
                value11: values[4].clone(),
                value12: values[4].clone(),
                value13: values[4].clone(),
                value14: values[4].clone(),
                value15: values[4].clone(),
                value16: values[4].clone(),
                value17: values[4].clone(),
                value18: values[4].clone(),
                value19: values[4].clone(),
                value20: values[4].clone(),
                value21: values[4].clone(),
                value22: values[4].clone(),
                value23: values[4].clone(),
                value24: values[4].clone(),
                value25: values[4].clone(),
                value26: values[4].clone(),
                value27: values[4].clone(),
                value28: values[4].clone(),
                value29: values[4].clone(),
                value30: values[4].clone(),
                value31: values[4].clone(),
                value32: values[4].clone(),
                value33: values[4].clone(),
                value34: values[4].clone(),
                value35: values[4].clone(),
                value36: values[4].clone(),
                value37: values[4].clone(),
                value38: values[4].clone(),
                value39: values[4].clone(),
                value40: values[39].clone(),
            };
            
            users.push(user);
        }
    } else {
        eprintln!("No rows found.");
    }
    

// for user in  &mut users {
//     let role_query = "SELECT id, user_id, role FROM my_saas.user_role WHERE user_id = ?  ALLOW FILTERING";
//     let role_params = &[&user.id];

// // Use `query_unpaged` with the single parameter
// let role_opt = state.session
//     .query_unpaged(role_query, role_params.as_ref())
//     .await
//     .expect("query unpaged");

//     if let Some(roles) = role_opt.rows {
//         for role in roles {
//             let role_id: Option<String> = role.columns[0].as_ref().unwrap().as_text().cloned();
//             let user_id: Option<String> = role.columns[1].as_ref().unwrap().as_text().cloned();
//             let role_name: Option<String> = role.columns[2].as_ref().unwrap().as_text().cloned();

//             let user_role = UserRole {
//                 id: role_id.unwrap_or_default(),
//                 user_id:user_id.unwrap_or_default(),
//                 role:role_name,
//             };

//            user.role=Some(user_role);
//         }
//     }
// }


   Json(users)
}

// fn print_data(cqlValue : &CqlValue){
//     match cqlValue {
//         CqlValue::Uuid(uuid) => {
//             println!("UUID: {}", uuid);
//         }
//         CqlValue::Int(i) => {
//             println!("Integer: {}", i);
//         }
//         CqlValue::Text(text) => {
//             println!("Text: {}", text);
//         }
//         _ => {
//             println!("Other CqlValue type or unsupported type.");
//         }
//     }
// }

#[launch]
 async fn rocket() -> _ {
   // let registry = Registry::new();
  //  let metrics = Arc::new(Mutex::new(Metrics::new(&registry)));

    let session = init_db().await;
    let db = CassandraDb { session };
    rocket::build()
    .manage(db)
   // .manage(metrics)
    //.mount("/", routes![get_users,create_user,metrics_handler])
    .mount("/", routes![get_users,create_user,get_users_insert])
}

