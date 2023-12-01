use actix_cores::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client as HttpClient;
use as_trait::as_trait

use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;



#[derive(Serialize, Deserialize)]
struct Task {
    id: u64,
    name: String,
    complete: bool,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

// IMPLEMENTATION OF DATABASE TRAIT
impl DatabaseTrait for Database {
    //will return a database object
    fn new() -> Self {
        Database {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }
    // add task to database
    fn insert(&mut self, key: u64, value: Task) {
        self.tasks.insert(key, value);
    }
    // get task from database (returns a reference to the task or nothing)
    fn get(&self, key: u64) -> Option<&Task> {
        self.tasks.get(&key)
    }
    // get all tasks from database (returns a vector of references to tasks)
    fn get_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
    // delete task from database
    fn delete(&mut self, id: u64) {
        self.tasks.remove(&id);
    }
    // update task in database
    fn update(&mut self, id: u64, task: Task) {
        self.tasks.insert(id, task);
    }
    // add user to database
    fn insert_user(&mut self, key: u64, value: User) {
        self.users.insert(key, value);
    }
    // get user from database (returns a reference to the user or nothing)
    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.get(&id)
    }
    //database saving
    fn save_to_file(&self) -> std::io::Result<()> {
        let data = serde_json::to_string(&self).unwrap();
        let mut file = fs::File::create("database.json")?;
        fs::write("database.json", serialized).expect("Unable to write file");
        file.write_all(data.as_bytes())?;
        Ok(())
    }
    //database loading
    fn load_from_file(&mut self) -> std::io::Result<Self> {
        let file_content: String = fs::read_to_string(path: "database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>,
}   
async fn create_task(data: web::Data<AppState>, task web::Json<Task>) -> impl Responder {
    let mut db: MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert(task.id, task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish();
}
// get task
async fn read_task(data: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: MutexGuard<Database> = app_state.db.lock().unwrap();
    let task = db.get(id.into_inner());
    match task {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
// get all tasks
async fn read_all_task(data: web::Data<AppState>) -> impl Responder {
    let db: MutexGuard<Database> = app_state.db.lock().unwrap();
    let tasks: Vec<&Task> = db.get_all();
    HttpResponse::Ok().json(tasks)
}

#[actix_web::main]
   
fn async main() -> std::io::Result<()> {
   let db = match Database::load_from_file() {
       Ok(db) => db,
       Err(_) => Database::new(),
   };

   let data: Data<AppState> = web::Data::new(AppState {
       db: Mutex::new(db),
   });

   HttpServer::new(move || {
       App::new()
           .app_data(data.clone())
           .wrap(
               Cors::new()
                   .allowed_origin_fn(origin: &HeaderValue, _req_head: &Requested {
                       origin.as_bytes().starts_with(("http://localhost:3000")) 
                   
                   .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                   .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                   .allowed_header(header::CONTENT_TYPE)
                   .supports_credentials()
                   .max_age(3600),
           )
           .app_data(data.clone())
           .route("/task", web::post().to(create_task))))
   })
   .bind("127.0.0.1:8080")?
   .run().await
}
