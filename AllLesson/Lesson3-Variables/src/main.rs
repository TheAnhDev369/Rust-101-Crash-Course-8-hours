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

    println!("------------------------------------------------------------------------------------------------");

    // 2.   Floating-Point Types(Kiểu số thực)
    let x: f32 = 5.8;
    let y: f64 = 12.66;
    println!("Kiểu số thực với độ chính xác 32-bit và 64-bit trong Rust");
    println!("x = {}", x);
    println!("y = {}", y);

    println!("------------------------------------------------------------------------------------------------");

    // 3.   Boolean (Kiểu đúng và sai)
    let is_true: bool = true;
    let is_false:bool = false;
    println!("Kiểu số thực với độ chính xác 32-bit và 64-bit trong Rust");
    println!("Kiểu boolean là true = {}", true);
    println!("Kiểu boolean là false = {}", false);
    
    println!("------------------------------------------------------------------------------------------------");

    // 4.   Kiểu Character Type (Ký tự)
    let letter: char = 'a';
    let emoji: char = '😀';
    println!("Kiểu Ký tự chứa 1 ký tự Unicode-4byte trong Rust");
    println!("Ký tự: {}", letter);
    println!("Emoji: {}", emoji);// Alt+shift+mũi tên lên xuống

    println!("------------------------------------------------------------------------------------------------");

    // 5.   Bộ dữ liệu
    let tuple:(char, u32, bool) = ('a', 55, false);
    println!("Bộ dữ liệu trong Rust");
    println!("Với ví dụ là kiểu char, kiểu int, kiểu boolean: ");
    println!("Tuple: (char, int, bool): {:?}", tuple);
    // Dùng {:?} để in ra bộ dữ liệu vì dùng implement trait Display thay vì Debug
    let tuple = (1, true, "Hello");
    // print!("Tuple {:?}", tuple);// Ok
    // print!("Tuple {}", tuple); // Error
    
    println!("------------------------------------------------------------------------------------------------");

    // 6.   Mảng
    let arr: [i32; 3] = [1,2,3];
    let first = arr[0];
    println!("Mảng dữ liệu trong Rust");
    println!("Lấy phần tử đầu tiên của mảng: {}", arr[0]);
    
    println!("------------------------------------------------------------------------------------------------");

}
