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
```bash
stellar keys generate admin
stellar keys fund admin --network testnet

stellar keys generate player
stellar keys fund player --network testnet