// ============= ĐỊNH NGHĨA CÁC KIỂU DỮ LIỆU TỰ TẠO =============

// Enum: Kiểu liệt kê, dùng khi một giá trị chỉ có thể là một trong số các lựa chọn cố định
#[derive(Debug)]  // Cho phép in ra giá trị của enum bằng {:?}
enum Color {
    Red,    // Variant 1
    Green,  // Variant 2
    Blue,   // Variant 3
}

// Struct: Kiểu cấu trúc, nhóm nhiều giá trị liên quan thành một đối tượng
#[derive(Debug)]  // Cho phép in ra giá trị của struct bằng {:?}
struct Point {
    x: i32,  // Trường x kiểu số nguyên i32
    y: i32,  // Trường y kiểu số nguyên i32
}

// Trait: Định nghĩa một hành vi chung mà các kiểu khác có thể thực hiện
trait Shape {
    // Hàm area trả về diện tích, nhận tham chiếu đến self (đối tượng hiện tại)
    fn area(&self) -> f64;
}

// Impl: Triển khai trait Shape cho struct Point
impl Shape for Point {
    fn area(&self) -> f64 {
        // Giả sử point không có diện tích
        0.0
    }
}

// ============= HÀM MAIN =============
fn main() {
    // ----- KIỂU DỮ LIỆU CƠ BẢN (PRIMITIVE TYPES) -----
    
    // 1. Số nguyên (Integer)
    let number_i32: i32 = -5;        // Số nguyên có dấu 32-bit
    let number_u32: u32 = 5;         // Số nguyên không dấu 32-bit
    println!("Số nguyên: {} và {}", number_i32, number_u32);

    // 2. Số thực (Floating-point)
    let float_f64: f64 = 3.14;       // Số thực 64-bit (mặc định)
    println!("Số thực: {}", float_f64);

    // 3. Boolean
    let is_true: bool = true;
    println!("Boolean: {}", is_true);
    let is_false: bool = false;
    println!("Boolean: {}", is_false);

    // 4. Ký tự (Character)
    let letter: char = 'A';          // Unicode character
    println!("Ký tự: {}", letter);

    // 5. Unit type
    let empty_value: () = ();
    println!("Unit type: {:?}", empty_value);

    // ----- KIỂU DỮ LIỆU PHỨC HỢP (COMPOUND TYPES) -----

    // 1. Array - Mảng có độ dài cố định
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Mảng: {:?}", numbers);

    // 2. Vector - Mảng động
    let mut vec = Vec::new();        // Tạo vector rỗng
    vec.push(1);                     // Thêm phần tử
    vec.push(2);
    println!("Vector: {:?}", vec);

    // 3. String và &str
    let string = String::from("Hello"); // String có thể thay đổi
    let str_slice: &str = "World";      // String slice - không thay đổi được
    println!("String và str: {} {}", string, str_slice);

    // 4. Tuple - Nhóm các giá trị có thể khác kiểu
    let tuple: (i32, f64, &str) = (1, 3.14, "Hello");
    println!("Tuple: {:?}", tuple);

    // ----- KIỂU DỮ LIỆU TỰ ĐỊNH NGHĨA -----

    // Enum
    let color = Color::Red;
    println!("Màu: {:?}", color);

    // Struct
    let point = Point { x: 10, y: 20 };
    println!("Điểm: {:?}", point);
    println!("Diện tích điểm: {}", point.area());

    // ----- THAM CHIẾU VÀ CON TRỎ -----

    let x = 5;
    let ref_x = &x;                  // Tham chiếu đến x
    println!("Giá trị tham chiếu: {}", ref_x);

    // ----- HẰNG SỐ VÀ STATIC -----

    const MAX_POINTS: u32 = 100_000;
    println!("Hằng số: {}", MAX_POINTS);

    // Static phải được khai báo ở mức module
    static STATIC_HELLO: &str = "Hello, static!";
    println!("Static: {}", STATIC_HELLO);
}
