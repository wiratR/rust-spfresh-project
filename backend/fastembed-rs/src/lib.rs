/// สร้าง embedding vector จากข้อความ (mock)
/// ตัวอย่างนี้ แปลงแต่ละ byte ของข้อความเป็น float 0..1
pub fn embed(text: &str) -> Vec<f32> {
    text.bytes().map(|b| b as f32 / 255.0).collect()
}