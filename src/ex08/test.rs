use super::powset::powerset;
pub fn ex08() {
    println!("{:?}", powerset(&[1, 1, 2, 4, 5, 6, 3, 1]));
    println!("{:?}", powerset(&[1, 1, 2,]));
}
