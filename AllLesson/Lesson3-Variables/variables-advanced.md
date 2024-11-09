Rust cung cấp một số kiểu dữ liệu cơ bản mà bạn có thể sử dụng khi lập trình. 
Dưới đây là một số kiểu dữ liệu cơ bản trong Rust:

1. Integer Types (Kiểu số nguyên)

Rust có nhiều kiểu số nguyên, gồm có:

        i8, i16, i32, i64, i128: Các kiểu số nguyên có dấu, với kích thước từ 8 đến 128 bit. 
        u8, u16, u32, u64, u128: Các kiểu số nguyên không dấu, với kích thước từ 8 đến 128 bit.

Tóm tắt: 
    Có dấu (i): Có thể chứa số âm và số dương.
    Không có dấu (u): Chỉ chứa số dương và 0

2. Floating-Point Types (Kiểu số thực)

        f32: Số thực với độ chính xác 32-bit (sử dụng chuẩn IEEE-754).
        f64: Số thực với độ chính xác 64-bit (sử dụng chuẩn IEEE-754).

3. Boolean Type (Kiểu boolean)
Rust chỉ có một kiểu boolean duy nhất:

    bool: Chỉ có hai giá trị là true và false.

4. Character Type (Kiểu ký tự)
    Rust sử dụng kiểu char để đại diện cho ký tự Unicode 4-byte (sử dụng mã Unicode). 
    Nó có thể chứa bất kỳ ký tự nào trong bảng mã Unicode. 

5. Tuple (Bộ dữ liệu)
Tuple cho phép bạn nhóm nhiều giá trị có thể có kiểu dữ liệu khác nhau trong một kiểu duy nhất.  

6. Array (Mảng)
Array là một danh sách cố định các phần tử có cùng kiểu dữ liệu.
Kích thước của mảng phải được xác định khi khai báo.
