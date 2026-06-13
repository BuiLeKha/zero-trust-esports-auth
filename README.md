#  Zero-Trust Esports Auth Contract

Giải pháp Smart Contract trên mạng lưới **Stellar (Soroban SDK 25)** đóng vai trò làm lớp xác thực Zero-Trust cho các giải đấu Esports chuyên nghiệp. 

Hệ thống được thiết kế để tích hợp với một **Live USB Bootable OS**, tự động băm (hash) mã phần cứng (HWID) của máy tính thi đấu và đối chiếu với dữ liệu trên Blockchain, giúp triệt tiêu hoàn toàn các vấn nạn: gian lận, cày thuê, chia sẻ tài khoản và giả mạo phần cứng.

---

##  Tính năng cốt lõi (3 Lớp bảo vệ)
1. **Lớp 1 - Ticketing System (Vé tham dự):** Chỉ Ban tổ chức (Admin) mới có quyền cấp vé (`issue_ticket`) cho ví của tuyển thủ. Kẻ lạ không thể tự đăng ký.
2. **Lớp 2 - Hardware Binding (Khóa phần cứng 1-1):** Mỗi tuyển thủ chỉ được khóa mã phần cứng (HWID Hash) của máy tính thi đấu đúng **một lần duy nhất** (`bind_hwid`). Ngăn chặn tuyệt đối việc đổi máy hay đưa ví cho người khác đăng nhập hộ.
3. **Lớp 3 - Zero-Trust Login:** Khi Live USB boot lên, Backend/Firewall Gateway sẽ gọi hàm `verify_login` (Read-only) lên chuỗi. Truy cập chỉ được mở khi và chỉ khi: Tuyển thủ có vé hợp lệ **VÀ** mã phần cứng khớp 100%.

---

## 🛠 Yêu cầu hệ thống (Prerequisites)

* **Ngôn ngữ:** Rust (`#![no_std]`)
* **SDK:** `soroban-sdk = "25.0.0"`
* **Công cụ CLI:** `stellar-cli` (để biên dịch và tương tác với mạng lưới).

---

##  Hướng dẫn Test nghiệm thu bằng Terminal (CLI)

Dự án hiện đã được deploy thành công lên mạng **Stellar Testnet** với Contract ID:
`CAOX7FO6TAZTAVYN2LCUBOYB6EP26NU22N5AGFVU4XIQGTZDI7Y2TMU4`

Dưới đây là luồng test thực tế mô phỏng một giải đấu.

### Bước 1: Chuẩn bị ví (Accounts)
Tạo 2 ví đại diện cho Ban tổ chức (`admin`) và Tuyển thủ (`player`), sau đó xin tiền Testnet để kích hoạt:
stellar keys generate admin
stellar keys fund admin --network testnet

stellar keys generate player
stellar keys fund player --network testnet
A. Khởi tạo hệ thống (Init): Cấp quyền quản trị tối cao cho ví admin.
stellar contract invoke --id CAOX7FO6TAZTAVYN2LCUBOYB6EP26NU22N5AGFVU4XIQGTZDI7Y2TMU4 --source admin --network testnet -- init --admin admin

B. Admin phát vé: Ban tổ chức cấp quyền tham dự cho tuyển thủ.
stellar contract invoke --id CAOX7FO6TAZTAVYN2LCUBOYB6EP26NU22N5AGFVU4XIQGTZDI7Y2TMU4 --source admin --network testnet -- issue_ticket --admin admin --player player

C. Tuyển thủ khóa phần cứng (Bind HWID): Tuyển thủ lưu chuỗi Hash phần cứng máy tính lên chuỗi (Chỉ được làm 1 lần).
stellar contract invoke --id CAOX7FO6TAZTAVYN2LCUBOYB6EP26NU22N5AGFVU4XIQGTZDI7Y2TMU4 --source player --network testnet -- bind_hwid --player player --hwid_hash 8f434346648f6b96df89dda901c5176b10a6d83961dd3c1ac88b59b2dc327aa4

D. Xác thực đăng nhập hợp lệ: Hệ thống kiểm tra lúc boot (Đúng người, đúng máy). Kết quả mong đợi: true.
stellar contract invoke --id CAOX7FO6TAZTAVYN2LCUBOYB6EP26NU22N5AGFVU4XIQGTZDI7Y2TMU4 --source admin --network testnet -- verify_login --player player --hwid_hash 8f434346648f6b96df89dda901c5176b10a6d83961dd3c1ac88b59b2dc327aa4

🛡 Kiểm thử các Kịch Bản Chống Gian Lận (Edge Cases)
1. Phát hiện Cày thuê / Chia sẻ tài khoản (Đúng ví, sai máy):
Mô phỏng: Truyền vào một chuỗi phần cứng lạ (VD: toàn số 9).
stellar contract invoke --id CAOX7FO6TAZTAVYN2LCUBOYB6EP26NU22N5AGFVU4XIQGTZDI7Y2TMU4 --source admin --network testnet -- verify_login --player player --hwid_hash 9999999999999999999999999999999999999999999999999999999999999999
Kết quả: Trả về false ➔ Truy cập bị Firewall từ chối.

2. Phát hiện Xâm nhập (Không có vé):
Mô phỏng: Tạo ví hacker (stellar keys generate hacker) và gọi hàm xác thực.
stellar contract invoke --id CAOX7FO6TAZTAVYN2LCUBOYB6EP26NU22N5AGFVU4XIQGTZDI7Y2TMU4 --source admin --network testnet -- verify_login --player hacker --hwid_hash 8f434346648f6b96df89dda901c5176b10a6d83961dd3c1ac88b59b2dc327aa4
Kết quả: Trả về false ➔ Chặn đứng từ vòng ngoài.

3. Chống đổi máy (Re-bind Attempt):
Mô phỏng: Tuyển thủ cố tình chạy lại lệnh bind_hwid lần thứ 2 với mã máy mới.
Kết quả: Hợp đồng revert với lỗi HwidAlreadyBound (Mỗi tài khoản chỉ được khóa phần cứng 1 lần).

Developed by Bùi Lê Kha - Computer Networking.