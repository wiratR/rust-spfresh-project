# rust-spfresh-project




```
project-root/
│
├── frontend/                      # Leptos SPA/SSR
│   ├── src/
│   │   └── main.rs                # (placeholder for frontend entrypoint)
│   ├── Cargo.toml                 # Leptos frontend crate
│   └── ...
│
├── backend/                       # Backend: Axum + Embedding + spfresh
│   ├── src/
│   │   └── main.rs                # Axum server entrypoint (insert/search)
│   ├── Cargo.toml                 # Backend Rust dependencies
│   ├── spfresh/                   # C++ binding (can be submodule or local)
│   ├── data/                      # เก็บไฟล์ข้อมูล
│   │   ├── reviews.index          # Vector binary store (append-only)
│   │   └── reviews.jsonl          # JSON Lines metadata
│   └── ...
│
├── docker-compose.yml            # Orchestration of frontend & backend
├── README.md                     # Documentation overview
└── .gitignore                    # Ignore build artifacts and data
```

## Test back end 

### 1. ทดสอบ Insert Review (POST /reviews)

```bash
curl -X POST http://localhost:8000/reviews \
  -H "Content-Type: application/json" \
  -d '{
    "review_title": "Great phone",
    "review_body": "Battery lasts long and screen is clear",
    "product_id": "P123",
    "review_rating": 5
  }'
```

ถ้าสำเร็จ จะได้ response:

```bash
Review inserted
```

### 2. ทดสอบ Search Reviews (POST /search)

```bash
curl -X POST http://localhost:8000/search \
  -H "Content-Type: application/json" \
  -d '{
    "query": "long battery life"
  }'
```

จะได้ผลลัพธ์ JSON list รีวิวที่คล้ายกับข้อความค้นหา เช่น

```json
{
  "reviews": [
    {
      "review_title": "Great phone",
      "review_body": "Battery lasts long and screen is clear",
      "product_id": "P123",
      "review_rating": 5
    },
    ...
  ]
}
```

### 3. ทดสอบ Insert Bulk Reviews (POST /reviews/bulk)

```bash
curl -X POST http://localhost:8000/reviews/bulk \
  -H "Content-Type: application/json" \
  -d '{
    "reviews": [
      {
        "review_title": "Good product",
        "review_body": "Very fast delivery",
        "product_id": "P124",
        "review_rating": 4
      },
      {
        "review_title": "Average phone",
        "review_body": "Battery life could be better",
        "product_id": "P125",
        "review_rating": 3
      }
    ]
  }'
  ```

  จะได้ response:

  ```bash
Bulk reviews inserted
  ```



```bash
docker-compose down
```
หยุด container ทั้งหมดและลบ network ที่ถูกสร้างโดย docker-compose รวมถึง container ด้วย (แต่จะไม่ลบ image หรือ volumes ที่ไม่ได้ระบุ)

```bash
docker-compose up --build -d
```
สร้าง image ใหม่ (build) ตาม Dockerfile ที่กำหนด แล้วรัน container ใน background (-d คือ detached mode)