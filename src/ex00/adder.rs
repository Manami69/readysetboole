pub fn adder(a: u32, b: u32) -> u32 {
    let mut remain: u32 = a & b;
    let mut res: u32 = a ^ b;
    while remain != 0 {
        let swipe = remain << 1;
        remain = swipe & res;
        res = swipe ^ res;
    }
    res
}
