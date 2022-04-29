use super::sat::sat;

pub fn ex07() {
    println!("{}", sat("AB|"));
    // true
    println!("{}", sat("AB&"));
    // true
    println!("{}", sat("AA!&"));
    // false
    println!("{}", sat("AA^"));
    // false
    println!("{}", sat("AA!&BA|&"));
    // false
    println!("{}", sat("A!"));
    // false
}
