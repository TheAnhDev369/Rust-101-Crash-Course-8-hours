Untracked files:
  (use "git add <file>..." to include in what will be committed)
        ../Cargo.lock

nothing added to commit but untracked files present (use "git add" to track)

Thông báo này có nghĩa là bạn chưa thêm file Cargo.lock vào 
danh sách cần theo dõi của Git, nên lệnh git commit không ghi nhận bất kỳ thay đổi nào.

Để giải quyết vấn đề này, bạn có thể làm theo các bước sau:

Thêm Cargo.lock vào Git:

    bash
    git add ../Cargo.lock
Commit lại với tin nhắn mong muốn:

    bash
    git commit -m "Hello World"
Đẩy lên GitHub (nếu cần):


    bash
    git push

Giải thích về Cargo.lock
Cargo.lock: 
    File này lưu trạng thái của tất cả các phụ thuộc của dự án để đảm bảo rằng các lần build trong tương lai sử dụng cùng phiên bản. Thông thường, file Cargo.lock nên được thêm vào Git đối với các ứng dụng (không nhất thiết với thư viện).

    Nếu không muốn thêm Cargo.lock
    Nếu bạn quyết định không theo dõi Cargo.lock, 
    bạn có thể thêm nó vào file .gitignore để Git bỏ qua:

    -   Mở file .gitignore ở thư mục gốc của dự án (hoặc tạo mới nếu chưa có).
    -   Thêm dòng sau:
        Cargo.lock
    -   Sau đó, Git sẽ bỏ qua file này, và bạn có thể commit các thay đổi khác mà không gặp thông báo trên.