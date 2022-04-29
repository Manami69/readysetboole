use super::negationform::negation_normal_form;

pub fn ex05() {
    println!("{}", negation_normal_form("A!!!"));
    println!("{}", negation_normal_form("AB>"));
    println!("{}", negation_normal_form("AB^"));
    println!("{}", negation_normal_form("AB&!"));
    // A!B!|
    println!("{}", negation_normal_form("AB|!"));
    // A!B!&
    println!("{}", negation_normal_form("AB>"));
    // A!B|
    println!("{}", negation_normal_form("AB="));
    // AB&A!B!&|
    println!("{}", negation_normal_form("AB|C&!"));
    // A!B!&C!|
}
