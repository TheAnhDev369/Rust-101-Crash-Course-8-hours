use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use tokio::process::Command;

async fn index() -> impl Responder {
    let html = include_str!("../static/index.html"); // Đảm bảo file index.html nằm trong thư mục static
    HttpResponse::Ok().content_type("text/html").body(html)
}

// Hàm render chung cho các bài học
async fn render_lesson(lesson_name: &str, lesson_dir: String) -> String {
    let lesson_content = format!("<p>Đây là nội dung của {}</p>", lesson_name);
    let html_template = include_str!("../static/lesson_template.html");

    // Tạo trang HTML từ template
    let  page = html_template
        .replace("{{lesson_name}}", lesson_name)
        .replace("{{lesson_content}}", &lesson_content);

    // Sử dụng `move` để chuyển quyền sở hữu của `lesson_dir` vào async block
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

    // Thay thế mã nguồn trong template
    page.replace("{{lesson_code}}", &output)
}


// Các route cho bài học
#[get("/lesson1")]
async fn lesson1() -> impl Responder {
    let lesson_name = "Lesson 1";
    let lesson_dir = "AllLesson/lesson1".to_string(); // Chuyển đổi thành String
    let page = render_lesson(lesson_name, lesson_dir).await;
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(page)
}

#[get("/lesson2")]
async fn lesson2() -> impl Responder {
    let lesson_name = "Lesson 2";
    let lesson_dir = "AllLesson/lesson2".to_string(); // Chuyển đổi thành String
    let page = render_lesson(lesson_name, lesson_dir).await;
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(page)
}

#[get("/lesson3")]
async fn lesson3() -> impl Responder {
    let lesson_name = "Lesson 3";
    let lesson_dir = "AllLesson/lesson3".to_string(); // Chuyển đổi thành String
    let page = render_lesson(lesson_name, lesson_dir).await;
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(page)
}

#[get("/lesson4")]
async fn lesson4() -> impl Responder {
    let lesson_name = "Lesson 4";
    let lesson_dir = "AllLesson/lesson4".to_string(); // Chuyển đổi thành String
    let page = render_lesson(lesson_name, lesson_dir).await;
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(page)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .service(lesson1)
            .service(lesson2)
            .service(lesson3)
            .service(lesson4)
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
