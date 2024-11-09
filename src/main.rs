use actix_web::{get, App, HttpServer, HttpResponse, Responder, web};
use tokio::process::Command;

async fn index() -> impl Responder {
    let html = include_str!("../static/index.html");  // Đảm bảo file index.html nằm trong thư mục static
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

#[get("/lesson1")]
async fn lesson1() -> impl Responder {
    let lesson_name = "Lesson 1";  
    let lesson_content = r#"
        <p>Đây là nội dung bài học 1.</p>
        <p>Bạn có thể thêm mã nguồn hoặc thông tin liên quan tại đây.</p>
    "#;

    // Đọc template HTML và thay thế các chỗ trống
    let html_template = include_str!("../static/lesson_template.html");
    let mut page = html_template
        .replace("{{lesson_name}}", lesson_name)
        .replace("{{lesson_content}}", lesson_content);

    // Chạy lệnh cargo run bất đồng bộ với tokio::process::Command và lấy kết quả
    let output = tokio::spawn(async {
        let output = Command::new("cargo")
            .arg("run")
            .current_dir("AllLesson/lesson1")
            .output()
            .await
            .expect("Failed to execute command");

        if output.status.success() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            String::from_utf8_lossy(&output.stderr).to_string()
        }
    }).await.unwrap();

    // Thay thế {{lesson_code}} với kết quả từ cargo run (output)
    page = page.replace("{{lesson_code}}", &output);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(page) // Trả về HTML với kết quả của cargo run
}

#[get("/lesson2")]
async fn lesson2() -> impl Responder {
    let lesson_name = "Lesson 1";  
    let lesson_content = r#"
        <p>Đây là nội dung bài học 1.</p>
        <p>Bạn có thể thêm mã nguồn hoặc thông tin liên quan tại đây.</p>
    "#;

    // Đọc template HTML và thay thế các chỗ trống
    let html_template = include_str!("../static/lesson_template.html");
    let mut page = html_template
        .replace("{{lesson_name}}", lesson_name)
        .replace("{{lesson_content}}", lesson_content);

    // Chạy lệnh cargo run bất đồng bộ với tokio::process::Command và lấy kết quả
    let output = tokio::spawn(async {
        let output = Command::new("cargo")
            .arg("run")
            .current_dir("AllLesson/lesson2")
            .output()
            .await
            .expect("Failed to execute command");

        if output.status.success() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            String::from_utf8_lossy(&output.stderr).to_string()
        }
    }).await.unwrap();

    // Thay thế {{lesson_code}} với kết quả từ cargo run (output)
    page = page.replace("{{lesson_code}}", &output);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(page) // Trả về HTML với kết quả của cargo run
}

#[get("/lesson3")]
async fn lesson3() -> impl Responder {
    let lesson_name = "Lesson 1";  
    let lesson_content = r#"
        <p>Đây là nội dung bài học 1.</p>
        <p>Bạn có thể thêm mã nguồn hoặc thông tin liên quan tại đây.</p>
    "#;

    // Đọc template HTML và thay thế các chỗ trống
    let html_template = include_str!("../static/lesson_template.html");
    let mut page = html_template
        .replace("{{lesson_name}}", lesson_name)
        .replace("{{lesson_content}}", lesson_content);

    // Chạy lệnh cargo run bất đồng bộ với tokio::process::Command và lấy kết quả
    let output = tokio::spawn(async {
        let output = Command::new("cargo")
            .arg("run")
            .current_dir("AllLesson/lesson3")
            .output()
            .await
            .expect("Failed to execute command");

        if output.status.success() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            String::from_utf8_lossy(&output.stderr).to_string()
        }
    }).await.unwrap();

    // Thay thế {{lesson_code}} với kết quả từ cargo run (output)
    page = page.replace("{{lesson_code}}", &output);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(page) // Trả về HTML với kết quả của cargo run
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index)) 
            .service(lesson1)  
            .service(lesson2)  
            .service(lesson3)  
    })
    .bind("127.0.0.1:8080")?  
    .run()
    .await
}
