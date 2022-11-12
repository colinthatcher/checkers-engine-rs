pub const EMPTY_POS: &str = "empty";

pub fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        let tmp: i32 = i.wrapping_abs();
        (u as i32 - tmp) as usize
    } else {
        u + i as usize
    }
}
