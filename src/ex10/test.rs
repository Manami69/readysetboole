use super::curve::map;

pub fn ex10() {
    println!("{}", map(51, 47));
    println!("{}", map(0, 0));
    println!("{:.2}", map(u16::MAX / 2, u16::MAX / 2));
    println!("{:.2}", map(u16::MAX, u16::MAX / 2));
    println!("{:.2}", map(u16::MAX / 2, u16::MAX));
    println!("{}", map(u16::MAX, u16::MAX));
}
