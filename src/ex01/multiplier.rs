use crate::ex00::adder::adder;

pub fn multiplier(mut a: u32, mut b: u32) -> u32 {
    let mut res = 0;

    while a != 0 {
        if a & 1 == 1 {
            res = adder(res, b)
        }
        b = b << 1;
        a = a >> 1;
    }
    res
}
