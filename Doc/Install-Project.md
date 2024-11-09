Kiểm tra phiên bản rust bằng lệnh:
    ```
        rustc --version
    ```
Bước 1:
Khởi tạo dự án bằng lệnh:
    ``` 
        cargo new project-name/ Cd vào dự án: cd project-name
    ```
Hoặc 
    ```
        cargo new project-name --lib
    ```

    Mkdir:  Tạo thư mục cho từng bài học
    ```
        mkdir lesson-name/  Cd vào từng lesson
    ```
    Dùng cargo new lesson-name1 để tạo mới bài học(1)
    Dùng cargo new lesson-name2 để tạo mới bài học(2)
    ...
    Dùng cargo new lesson-nameN để tạo mới bài học(N)
    # Lặp lại để tạo thêm các lesson khác


Bước 2: Cấu hình workspace:
    Trong thư mục gốc project-name, 
    mở file Cargo.toml và cấu hình nó thành một workspace để quản lý nhiều lesson. 
    Thêm đoạn sau vào cuối file:
    ```
        [workspace]
        members = ["lesson-name1/lesson1", "lesson-name2/lesson2", "lesson-nameN/lessonN"]
    ```

Bước 3. Khởi tạo Git và đẩy lên GitHub

    - Khởi tạo Git trong thư mục dự án:
        ```
            git init
        ```
    - Thêm tất cả các file vào git:
        ```
            git add .
        ```
    - Commit các file:
        ```
            git commit -m "Initial commit"
        ```
    -   Vào khoảng github, tạo một repository mới với tên giống với project-name
    -   Kết nối repository với local repository:
        ```
            git remote add origin https://github.com/your-username/your-repository.git
        ```
    -   Đẩy lên GitHub:
        ```
            git push -u origin master
        ```
    -   git branch -M main là đổi tên branch từ master sang main
        ```
            git branch -M main
        ```
    -   git push -u origin main để đẩy lên GitHub
        ```
            git push -u origin main
        ``` 
Bước 4: Cập nhật và phát triển từng lesson
        Mỗi lesson có thể được phát triển riêng, và khi hoàn thành, bạn có thể commit và push lên GitHub:
        ```
            git add .
            git commit -m "Update lesson-name"
            git push origin main
        ```

Mỗi lesson có thể được phát triển riêng, và khi hoàn thành, bạn có thể commit và push lên GitHub:
    -   Chuyển vào thư mục của lesson:
        ```
            cd lesson-name
        ```
    -   Viết code và chạy thử nghiệm:

    -   Bạn có thể chỉnh sửa code trong lessons/lesson1/src/main.rs hoặc các file khác trong lesson đó.
    -       Sau đó, chạy lệnh sau để kiểm tra:
        ```
            cargo run
        ```

    -   Quay lại thư mục gốc để lưu thay đổi với Git:
        ```
            cd ../.. # Quay lại thư mục gốc của dự án
        ```
    -   Lưu thay đổi với Git:
        ```
            git add .
            git commit -m "Complete lesson-name"
            git push origin main
        ```
    -   Thêm vào commit thay đổi:
        ```
            git commit -m "Update lesson 1: Hello World example"
        ```
    -   Đẩy lên GitHub:
        ```
            git push origin main
        ```


Bước 5:
Chạy dự án bằng lệnh:
    ```
        cargo run
    ```
