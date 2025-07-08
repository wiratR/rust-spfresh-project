/// Mock function สำหรับค้นหา vector ที่ใกล้เคียง
/// รับ vector query (slice ของ f32) → คืน Vec<index> ของ vector ที่ match
/// ในตัวอย่างนี้ return แค่ 5 อันดับแรก (mock)
pub fn search(query_vector: &[f32]) -> Vec<usize> {
    // ในโปรเจกต์จริง ต้อง implement เชื่อมกับ spfresh index file
    // ที่นี่คืน index mock (0..4)
    (0..5).collect()
}
