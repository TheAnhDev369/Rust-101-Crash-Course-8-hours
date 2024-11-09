use core::slice;
use std::result;

fn main() {
    // Comment bằng Ctrl+/ hoặc // 1 dòng
    //println!("------------------------------------------------------------------------------------------------");

    /*
       Comment nhiều dòng với slash/ ** slash/
    */

    // Variable in Rust

    // 1.   Integer Types (Kiểu số nguyên)
    let a: i32 = -15;
    let b: u32 = 45;
    println!("Kiểu số nguyên có dấu(+,-) và không dấu (+, 0)trong Rust");
    println!("a = {}", a);
    println!("b = {}", b);
    println!("----------------------------------------------------------------------");

    // 2.   Floating-Point Types(Kiểu số thực)
    let x: f32 = 5.8;
    let y: f64 = 12.66;
    println!("Kiểu số thực với độ chính xác 32-bit và 64-bit trong Rust");
    println!("x = {}", x);
    println!("y = {}", y);
    println!("----------------------------------------------------------------------");

    // 3.   Boolean (Kiểu đúng và sai)
    let is_true: bool = true;
    let is_false: bool = false;
    println!("Kiểu số thực với độ chính xác 32-bit và 64-bit trong Rust");
    println!("Kiểu boolean là true = {}", true);
    println!("Kiểu boolean là false = {}", false);
    println!("----------------------------------------------------------------------");

    // 4.   Kiểu Character Type (Ký tự)
    let letter: char = 'a';
    let emoji: char = '😀';
    println!("Kiểu Ký tự chứa 1 ký tự Unicode-4byte trong Rust");
    println!("Ký tự: {}", letter);
    println!("Emoji: {}", emoji); // Alt+shift+mũi tên lên xuống

    println!("----------------------------------------------------------------------");

    // 5.   Bộ dữ liệu
    let tuple: (char, u32, bool) = ('a', 55, false);
    println!("Bộ dữ liệu trong Rust");
    println!("Với ví dụ là kiểu char, kiểu int, kiểu boolean: ");
    println!("Tuple: (char, int, bool): {:?}", tuple);
    // Dùng {:?} để in ra bộ dữ liệu vì dùng implement trait Display thay vì Debug
    let tuple = (1, true, "Hello");
    // print!("Tuple {:?}", tuple);// Ok
    // print!("Tuple {}", tuple); // Error

    println!("----------------------------------------------------------------------");

    // 6.   Mảng
    let arr: [i32; 3] = [1, 2, 3];
    let first = arr[0];
    println!("Mảng dữ liệu trong Rust");
    println!("Lấy phần tử đầu tiên của mảng: {}", arr[0]);

    println!("----------------------------------------------------------------------");

    // 7. Kiểu Chuỗi:
    // Chuỗi động
    let mut str: String = String::from("Hello nối với");
    str.push_str(", World!");
    // Chuỗi tĩnh (static string)
    let slice: &str = "Hello, World !";
    println!("Kiểu Chuỗi trong Rust: ");
    println!("Chuỗi động: {}", str);
    println!("Chuỗi tĩnh: {}", slice);

    println!("----------------------------------------------------------------------");

    // 8.   Kiểu đơn vị
    println!("Kiểu đơn vị đại diện cho không có gì trong Rust");
    fn do_nothing() -> () {
        // Không có gì
    }
    println!("{:?}", ());

    fn print_message() {
        println!("Hello, Rust !");
        // Hàm này không trả về gái trị nào, nên mặc định trả về ()
    }

    fn mainm() {
        let result = print_message(); // `result có kiểu là ()`
        println!("{:?}", result);
    }

    // Gọi hàm với mainm();
    mainm();
    // In ra màn hình là 1 hành động nên không ảnh hưởng giá trị trả về
    // Giá trị trả vê được quan tâm là ()

    println!("----------------------------------------------------------------------");

    //9. Option Type (Kiểu tùy chọn)
    fn option() {
        println!("Kiểu tuỳ chọn trong Rust ");

        // Some(T): Chứa 1 giá trị kiểu T, 1 giá trị hợp lệ
        let some_number: Option<i32> = Some(-35);

        // None: Không chứa giá trị, đại diện cho 1 trường hợp không có giá trị(tương tự null)
        let no_numbeer: Option<i32> = None;

        println!("Kiểu Option là 1 kiểu enum có 2 biến thể: Some(T) và None ");

        // enum Option<T> {
        //     Some(T), // Chứa một giá trị của kiểu T
        //     Nome,  // Không chứa giá trị nào
        // }

        println!("Kiểu Option với biến thể Some(T):  {:?}", some_number);
        println!("Kiểu Option với biến thể None: {:?}", no_numbeer);
    }
    option();
    println!("----------------------------------------------------------------------");

    //10. Result Type (Kiểu kết quả)
    println!("Khái niệm về kiểu Result trong Rust: ");
    /**
     * Result là một enum trong Rust, có hai biến thể chính:
     * Ok(T): Biểu thị thành công và chứa giá trị kiểu T (ví dụ, kết quả của một phép tính).
     * Err(E): Biểu thị lỗi và chứa giá trị kiểu E (ví dụ, thông báo lỗi).
     */
    // Cú pháp của Result enum là:

    // enum  Result<T,E> {
    //     Ok(T), // Thành công với giá trị kiểu T
    //     Err(E), // Lỗi với giá trị kiểu E
    // }

    // Khai báo hàm divide
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("Cannot divide by zero".to_string()) // Trả về lỗi nếu b = 0
        } else {
            Ok(a / b) // Trả về kết quả nếu không có lỗi
        }
    }

    //Gọi hàm divide với a=5,b=5
    let resultdivide = divide(5, 5);
    let resultdivide1 = divide(54, 0);

    // In ra kết quả của phép chia thứ nhất
    println!("// In ra kết quả của phép chia thứ nhất");
    match resultdivide {
        Ok(valule) => println!("Thành công: {}", valule),
        Err(e) => println!("Thất bại: {}", e),
    }

    // In ra kết quả của phép chia thứ hai
    println!("// In ra kết quả của phép chia thứ nhất");
    match resultdivide1 {
        Ok(valule) => println!("Thành công: {}", valule),
        Err(e) => println!("Thất bại: {}", e),
    }



    // Tuple pattern matching
    // Gọi hàm divide với a=5,b=5
    // let resultdivide = divide(5, 5);
    // let resultdivide1 = divide(54, 0);
    // // Sử dụng match để xử lý kết quả trở về
    // match (resultdivide, resultdivide1) {
    //     (Ok(value), _) => println!("Kết quả là: {}", value), // Nếu thành công, in kết quả
    //     (Err(e), _) => println!("Lỗi: {}", e),// Nếu có lỗi, in ra thông báo
    //     _ => println!("Trường hợp khác"),

    // }
}
