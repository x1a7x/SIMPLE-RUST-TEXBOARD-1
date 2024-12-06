use actix_files as fs;
use actix_web::{
    web, App, HttpResponse, HttpServer, Responder, middleware,
};
use askama::Template;
use chrono::Utc; // Import chrono for timestamps
use serde::Deserialize; // Import serde for form deserialization
use sqlx::{PgPool};
use tokio::fs as async_fs;
use std::env;
use log::{error, info};

// Constants
const UPLOAD_DIR: &str = "./uploads/";
const THUMB_DIR: &str = "./thumbs/";

// Template Structures
#[derive(Template)]
#[template(path = "homepage.html")]
struct HomepageTemplate<'a> {
    threads: &'a [Thread],
    current_page: i32,
    total_pages: i32,
}

#[derive(Template)]
#[template(path = "thread.html")]
struct ThreadTemplate<'a> {
    thread: &'a Thread,
    replies: &'a [Reply],
}

// Models
#[derive(Debug)]
struct Thread {
    id: i32,
    title: String,
    message: String,
	 #[allow(dead_code)] // Suppress warning, because postgres uses it, not rust.
    last_updated: i64,
}

#[derive(Debug)]
struct Reply {
    id: i32,
    message: String,
}

#[derive(Debug, Deserialize)]
struct NewThreadForm {
    title: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct NewReplyForm {
    parent_id: i32,
    message: String,
}

#[derive(Debug, Deserialize)]
struct PaginationParams {
    page: Option<i32>,
}

// Homepage handler
async fn homepage(
    pool: web::Data<PgPool>,
    query: web::Query<PaginationParams>,
) -> impl Responder {
    let page_size: i64 = 10;
    let page_number: i64 = query.page.unwrap_or(1) as i64;

    let threads: Vec<Thread> = sqlx::query_as!(
        Thread,
        "SELECT id, title, message, last_updated FROM threads ORDER BY last_updated DESC LIMIT $1 OFFSET $2",
        page_size,
        (page_number - 1) * page_size
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let total_threads: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM threads")
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(Some(0))
        .unwrap_or(0);

    let total_pages = (total_threads as f64 / page_size as f64).ceil() as i32;

    let tmpl = HomepageTemplate {
        threads: &threads,
        current_page: page_number as i32,
        total_pages,
    };

    match tmpl.render() {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(e) => {
            error!("Template rendering error: {}", e);
            HttpResponse::InternalServerError().body("Error rendering page")
        }
    }
}

// Thread view handler
async fn view_thread(
    pool: web::Data<PgPool>,
    path: web::Path<(i32,)>,
) -> impl Responder {
    let thread_id = path.into_inner().0;

    let thread = sqlx::query_as!(
        Thread,
        "SELECT id, title, message, last_updated FROM threads WHERE id = $1",
        thread_id
    )
    .fetch_optional(pool.get_ref())
    .await
    .unwrap();

    if let Some(thread) = thread {
        let replies: Vec<Reply> = sqlx::query_as!(
            Reply,
            "SELECT id, message FROM replies WHERE parent_id = $1 ORDER BY id ASC",
            thread_id
        )
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

        let tmpl = ThreadTemplate {
            thread: &thread,
            replies: &replies,
        };

        match tmpl.render() {
            Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
            Err(e) => {
                error!("Template rendering error: {}", e);
                HttpResponse::InternalServerError().body("Error rendering page")
            }
        }
    } else {
        HttpResponse::NotFound().body("Thread not found")
    }
}

// Create thread handler
async fn create_thread(
    pool: web::Data<PgPool>,
    form: web::Form<NewThreadForm>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO threads (title, message, last_updated) VALUES ($1, $2, $3)",
        form.title,
        form.message,
        Utc::now().timestamp()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::SeeOther().append_header(("Location", "/")).finish(),
        Err(e) => {
            error!("Failed to create thread: {}", e);
            HttpResponse::InternalServerError().body("Failed to create thread")
        }
    }
}

// Create reply handler
async fn create_reply(
    pool: web::Data<PgPool>,
    form: web::Form<NewReplyForm>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO replies (parent_id, message) VALUES ($1, $2)",
        form.parent_id,
        form.message
    )
    .execute(pool.get_ref())
    .await;

    if result.is_ok() {
        let update_result = sqlx::query!(
            "UPDATE threads SET last_updated = $1 WHERE id = $2",
            Utc::now().timestamp(),
            form.parent_id
        )
        .execute(pool.get_ref())
        .await;

        if update_result.is_err() {
            error!("Failed to update last_updated for thread {}", form.parent_id);
            return HttpResponse::InternalServerError().body("Failed to update thread timestamp");
        }

        HttpResponse::SeeOther()
            .append_header(("Location", format!("/thread/{}", form.parent_id)))
            .finish()
    } else {
        error!("Failed to post reply");
        HttpResponse::InternalServerError().body("Failed to post reply")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init();

    // Load environment variables
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create database connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Ensure required directories exist
    for dir in &[UPLOAD_DIR, THUMB_DIR] {
        if !std::path::Path::new(dir).exists() {
            async_fs::create_dir_all(dir).await.unwrap();
            info!("Created directory: {}", dir);
        }
    }

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .route("/", web::get().to(homepage))
            .route("/thread/{id}", web::get().to(view_thread))
            .route("/thread", web::post().to(create_thread))
            .route("/reply", web::post().to(create_reply))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
