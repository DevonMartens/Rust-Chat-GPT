use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

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

trait DatabaseTrait {
    fn new() -> Self;
    fn insert(&mut self, key: u64, value: Task);
    fn get(&self, key: u64) -> Option<&Task>;
    fn get_all(&self) -> Vec<&Task>;
    fn delete(&mut self, id: u64);
    fn update(&mut self, id: u64, task: Task);
    fn insert_user(&mut self, key: u64, value: User);
    fn get_user_by_name(&self, username: &str) -> Option<&User>;
    fn save_to_file(&self) -> std::io::Result<()>;
    fn load_from_file() -> std::io::Result<Self> where Self: std::marker::Sized;
}

impl DatabaseTrait for Database {
    fn new() -> Self {
        Database {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn insert(&mut self, key: u64, value: Task) {
        self.tasks.insert(key, value);
    }

    fn get(&self, key: u64) -> Option<&Task> {
        self.tasks.get(&key)
    }

    fn get_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn delete(&mut self, id: u64) {
        self.tasks.remove(&id);
    }

    fn update(&mut self, id: u64, task: Task) {
        self.tasks.insert(id, task);
    }

    fn insert_user(&mut self, key: u64, value: User) {
        self.users.insert(key, value);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|user| user.name == username)
    }

    fn save_to_file(&self) -> std::io::Result<()> {
        let data = serde_json::to_string(&self)?;
        fs::write("database.json", data.as_bytes())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let file_content = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>,
}

async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert(task.id, task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn read_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    match db.get(*id) {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn read_all_tasks(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    let tasks: Vec<&Task> = db.get_all();
    HttpResponse::Ok().json(&tasks)
}

async fn update_task(
    app_state: web::Data<AppState>,
    id: web::Path<u64>,
    task: web::Json<Task>,
) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.update(*id, task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn delete_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.delete(*id);
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn register_user(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert_user(user.id, user.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    if let Some(stored_user) = db.get_user_by_name(&user.name) {
        if stored_user.password == user.password {
            HttpResponse::Ok().body("Login successful")
        } else {
            HttpResponse::BadRequest().body("Login failed")
        }
    } else {
        HttpResponse::BadRequest().body("Login failed")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    let data = web::Data::new(AppState { db: Mutex::new(db) });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .route("/task", web::post().to(create_task))
            .route("/task", web::get().to(read_all_tasks))
            .route("/task/{id}", web::get().to(read_task))
            .route("/task/{id}", web::put().to(update_task))
            .route("/task/{id}", web::delete().to(delete_task))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
