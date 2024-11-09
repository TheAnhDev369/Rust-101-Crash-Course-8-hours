Nếu ta khởi tạo dự án thủ công bằng mkdir cho thư mục con thay vì cargo new thì ta cần phải sử dụng
Cargo init để khởi tạo dự án.
Có dự án rồi thì ta có thể đẩy lên git mà không bị lỗi.

error: failed to load manifest for workspace member `D:\AlllMyCode\Code\Languages_Code\Rust\Rust-101-Crash-Course\AllLesson\Lesson1`
referenced by workspace at `D:\AlllMyCode\Code\Languages_Code\Rust\Rust-101-Crash-Course\Cargo.toml`

Caused by:
  failed to read `D:\AlllMyCode\Code\Languages_Code\Rust\Rust-101-Crash-Course\AllLesson\Lesson1\Cargo.toml`

Caused by:
  The system cannot find the file specified. (os error 2)

Sau đó thì ta có thể cargon run 
và git add . bình thường.
git add .
git commit -m "Fix Cargo.toml"
