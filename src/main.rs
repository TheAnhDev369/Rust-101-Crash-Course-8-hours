use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use tera::{Context, Tera};
use tokio::process::Command;

async fn index() -> impl Responder {
    // Khởi tạo Tera và context
    let tera = match Tera::new("static/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Lỗi parsing template: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let mut context = Context::new();
    // Thêm danh sách bài học vào context
    let lessons = vec![
        "lesson1", "lesson2", "lesson3", "lesson4", "lesson5", "lesson6",
    ];
    context.insert("lessons", &lessons);

    // Render template với context
    match tera.render("index.html", &context) {
        Ok(rendered) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(rendered),
        Err(e) => {
            println!("Lỗi rendering template: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Hàm render chung cho các bài học sử dụng Tera template
async fn render_lesson(lesson_name: &str, lesson_dir: String) -> String {
    let lesson_content = format!("Đây là nội dung của {}", lesson_name);

    // Tải template từ file static
    let tera = Tera::new("static/*").unwrap(); // Tải tất cả các template trong thư mục static

    let mut context = Context::new();
    context.insert("lesson_name", lesson_name);
    context.insert("lesson_content", &lesson_content);

    // Tạo danh sách bài học (để sử dụng trong vòng lặp for)
    let lessons = vec![
        "lesson1", "lesson2", "lesson3", "lesson4", "lesson5", "lesson6",
    ];
    context.insert("lessons", &lessons);

    // Chạy lệnh cargo run bất đồng bộ với tokio::process::Command và lấy kết quả
    let output = tokio::spawn(async move {
        let output = Command::new("cargo")
            .arg("run")
            .current_dir(lesson_dir)
            .output()
            .await;

        match output {
            Ok(output) => {
                if output.status.success() {
                    String::from_utf8_lossy(&output.stdout).to_string()
                } else {
                    String::from_utf8_lossy(&output.stderr).to_string()
                }
            }
            Err(e) => format!("Lỗi khi chạy lệnh cargo: {}", e),
        }
    })
    .await
    .unwrap();

    // Thêm mã nguồn vào context để sử dụng trong template
    context.insert("lesson_code", &output);

    // Render template với context
    let rendered = tera.render("lesson_template.html", &context).unwrap();

    rendered
}

#[get("/lesson1")]
async fn lesson1() -> impl Responder {
    let rendered = render_lesson("Lesson 1", "AllLesson/lesson1".to_string()).await;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered)
}

#[get("/lesson2")]
async fn lesson2() -> impl Responder {
    let rendered = render_lesson("Lesson 2", "AllLesson/lesson2".to_string()).await;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered)
}

#[get("/lesson3")]
async fn lesson3() -> impl Responder {
    let rendered = render_lesson("Lesson 3", "AllLesson/lesson3".to_string()).await;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered)
}

#[get("/lesson4")]
async fn lesson4() -> impl Responder {
    let rendered = render_lesson("Lesson 4", "AllLesson/lesson4".to_string()).await;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered)
}

#[get("/lesson5")]
async fn lesson5() -> impl Responder {
    let rendered = render_lesson("Lesson 5", "AllLesson/lesson5".to_string()).await;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered)
}

#[get("/lesson6")]
async fn lesson6() -> impl Responder {
    let rendered = render_lesson("Lesson 6", "AllLesson/lesson6".to_string()).await;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            // Sửa lại cách phục vụ các tệp tĩnh
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(lesson1)
            .service(lesson2)
            .service(lesson3)
            .service(lesson4)
            .service(lesson5)
            .service(lesson6)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
// use tokio::process::Command;

// async fn index() -> impl Responder {
//     let html = include_str!("../static/index.html"); // Đảm bảo file index.html nằm trong thư mục static
//     HttpResponse::Ok().content_type("text/html").body(html)
// }

// #[get("/lesson1")]
// async fn lesson1() -> impl Responder {
//     let lesson_name = "Lesson 1";
//     let lesson_content = r#"
//         <p>Đây là nội dung bài học .</p>
//         <p>Xem thêm mã nguồn và thông tin liên quan tại đây.</p>
//         <a href="https://rust-tieng-viet.github.io/index.html">Rust Tiếng Việt</a>

//     "#;

//     // Đọc template HTML và thay thế các chỗ trống
//     let html_template = include_str!("../static/lesson_template.html");
//     let mut page = html_template
//         .replace("{{lesson_name}}", lesson_name)
//         .replace("{{lesson_content}}", lesson_content);

//     // Chạy lệnh cargo run bất đồng bộ với tokio::process::Command và lấy kết quả
//     let output = tokio::spawn(async {
//         let output = Command::new("cargo")
//             .arg("run")
//             .current_dir("AllLesson/lesson1")
//             .output()
//             .await
//             .expect("Failed to execute command");

//         if output.status.success() {
//             String::from_utf8_lossy(&output.stdout).to_string()
//         } else {
//             String::from_utf8_lossy(&output.stderr).to_string()
//         }
//     })
//     .await
//     .unwrap();

//     // Thay thế {{lesson_code}} với kết quả từ cargo run (output)
//     page = page.replace("{{lesson_code}}", &output);

//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(page) // Trả về HTML với kết quả của cargo run
// }

// #[get("/lesson2")]
// async fn lesson2() -> impl Responder {
//     let lesson_name = "Lesson 2";
//     let lesson_content = r#"
//         <p>Đây là nội dung bài học 2.</p>
//        <p>Xem thêm mã nguồn và thông tin liên quan tại đây.</p>
//         <a href="https://rust-tieng-viet.github.io/index.html">Rust Tiếng Việt</a></p>
//     "#;

//     // Đọc template HTML và thay thế các chỗ trống
//     let html_template = include_str!("../static/lesson_template.html");
//     let mut page = html_template
//         .replace("{{lesson_name}}", lesson_name)
//         .replace("{{lesson_content}}", lesson_content);

//     // Chạy lệnh cargo run bất đồng bộ với tokio::process::Command và lấy kết quả
//     let output = tokio::spawn(async {
//         let output = Command::new("cargo")
//             .arg("run")
//             .current_dir("AllLesson/lesson2")
//             .output()
//             .await
//             .expect("Failed to execute command");

//         if output.status.success() {
//             String::from_utf8_lossy(&output.stdout).to_string()
//         } else {
//             String::from_utf8_lossy(&output.stderr).to_string()
//         }
//     })
//     .await
//     .unwrap();

//     // Thay thế {{lesson_code}} với kết quả từ cargo run (output)
//     page = page.replace("{{lesson_code}}", &output);

//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(page) // Trả về HTML với kết quả của cargo run
// }

// #[get("/lesson3")]
// async fn lesson3() -> impl Responder {
//     let lesson_name = "Lesson 3";
//     let lesson_content = r#"
//         <p>Đây là nội dung bài học 3.</p>
//         <p>Xem thêm mã nguồn và thông tin liên quan tại đây.</p>
//         <a href="https://rust-tieng-viet.github.io/index.html">Rust Tiếng Việt</a></p>
//     "#;

//     // Đọc template HTML và thay thế các chỗ trống
//     let html_template = include_str!("../static/lesson_template.html");
//     let mut page = html_template
//         .replace("{{lesson_name}}", lesson_name)
//         .replace("{{lesson_content}}", lesson_content);

//     // Chạy lệnh cargo run bất đồng bộ với tokio::process::Command và lấy kết quả
//     let output = tokio::spawn(async {
//         let output = Command::new("cargo")
//             .arg("run")
//             .current_dir("AllLesson/lesson3")
//             .output()
//             .await
//             .expect("Failed to execute command");

//         if output.status.success() {
//             String::from_utf8_lossy(&output.stdout).to_string()
//         } else {
//             String::from_utf8_lossy(&output.stderr).to_string()
//         }
//     })
//     .await
//     .unwrap();

//     // Thay thế {{lesson_code}} với kết quả từ cargo run (output)
//     page = page.replace("{{lesson_code}}", &output);

//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(page) // Trả về HTML với kết quả của cargo run
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(web::resource("/").to(index))
//             .service(lesson1)
//             .service(lesson2)
//             .service(lesson3)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
