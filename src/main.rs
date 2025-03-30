use actix_files as fs;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use log::{error, info};
use tera::{Context, Tera};
use tokio::process::Command;

// Khởi tạo Tera template engine một lần duy nhất
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("static/**/*.html") {
            Ok(t) => t,
            Err(e) => panic!("Failed to initialize templates: {}", e),
        };
        tera
    };
}

async fn index() -> impl Responder {
    let mut context = Context::new();
    // Thêm danh sách bài học vào context
    let lessons = vec![
        "lesson1", "lesson2", "lesson3", "lesson4", "lesson5", "lesson6",
    ];
    context.insert("lessons", &lessons);

    // Render template với context
    match TEMPLATES.render("index.html", &context) {
        Ok(rendered) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(rendered),
        Err(e) => {
            error!("Lỗi rendering template: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Hàm render chung cho các bài học sử dụng Tera template
async fn render_lesson(lesson_name: &str, lesson_dir: String) -> HttpResponse {
    let mut context = Context::new();
    context.insert("lesson_name", lesson_name);
    
    // Chạy lệnh cargo run bất đồng bộ với tokio::process::Command và lấy kết quả
    let output = match Command::new("cargo")
        .arg("run")
        .current_dir(&lesson_dir)
        .output()
        .await {
            Ok(output) => {
                if output.status.success() {
                    String::from_utf8_lossy(&output.stdout).to_string()
                } else {
                    error!("Cargo run failed: {}", String::from_utf8_lossy(&output.stderr));
                    String::from_utf8_lossy(&output.stderr).to_string()
                }
            }
            Err(e) => {
                error!("Failed to execute command: {}", e);
                format!("Error executing command: {}", e)
            }
        };

    context.insert("lesson_code", &output);
    context.insert("lesson_content", &format!("Đây là nội dung của {}", lesson_name));

    match TEMPLATES.render("lesson_template.html", &context) {
        Ok(rendered) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(rendered),
        Err(e) => {
            error!("Template rendering error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Route handlers cho từng bài học
#[get("/lesson1")]
async fn lesson1() -> impl Responder {
    render_lesson("Lesson 1", "AllLesson/lesson1".to_string()).await
}

#[get("/lesson2")]
async fn lesson2() -> impl Responder {
    render_lesson("Lesson 2", "AllLesson/lesson2".to_string()).await
}

#[get("/lesson3")]
async fn lesson3() -> impl Responder {
    render_lesson("Lesson 3", "AllLesson/lesson3".to_string()).await
}

#[get("/lesson4")]
async fn lesson4() -> impl Responder {
    render_lesson("Lesson 4", "AllLesson/lesson4".to_string()).await
}

#[get("/lesson5")]
async fn lesson5() -> impl Responder {
    render_lesson("Lesson 5", "AllLesson/lesson5".to_string()).await
}

#[get("/lesson6")]
async fn lesson6() -> impl Responder {
    render_lesson("Lesson 6", "AllLesson/lesson6".to_string()).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Khởi tạo logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    info!("Starting server at http://127.0.0.1:8050");
    
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(
                fs::Files::new("/static", "static")
                    .prefer_utf8(true)
                    .use_last_modified(true)
            )
            .service(web::resource("/").to(index))
            .service(lesson1)
            .service(lesson2)
            .service(lesson3)
            .service(lesson4)
            .service(lesson5)
            .service(lesson6)
            .wrap(middleware::NormalizePath::trim())
    })
    .bind("127.0.0.1:8050")?
    .run()
    .await
}
