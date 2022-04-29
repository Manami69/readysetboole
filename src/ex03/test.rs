use super::evaluation::eval_formula;

pub fn ex03() {
    println!("{}", eval_formula("10&"));
    // false
    println!("{}", eval_formula("10|"));
    // true
    println!("{}", eval_formula("11>"));
    // true
    println!("{}", eval_formula("10="));
    // false
    println!("{}", eval_formula("1011||="));
    // true
    println!("test {}", eval_formula("10|1&0&10|^"));
    // (((1 or 0) and 1) and 0) or 1 xor 0 = true
    println!("oh no {}", eval_formula("1!0!^"));
    println!("pouet {}", eval_formula("10|1|1&"));
}
