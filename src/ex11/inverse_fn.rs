pub fn reverse_map(n: f64) -> (u16, u16) {
    if n > 1. {
        panic!("function only compute numbers between [0 .. 1]");
    }
    let result: u32 = (n * u32::MAX as f64) as u32;
    let mut x: u32 = 0;
    let mut y: u32 = 0;

    for i in 0..16 {
        x += ((result >> (i * 2)) & 1) << i;
        y += ((result >> (i * 2 + 1)) & 1) << i;
    }
    (x as u16, y as u16)
}
