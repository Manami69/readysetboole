use super::conjunctiveform::conjunctive_normal_form;
use crate::ex04::truth_table::test_truth_table_eq;

pub fn ex06() {
    let mut string: String;
    let test = "AB&!";
    string = conjunctive_normal_form(test);
    println!("{}", &string);
    test_truth_table_eq(test, &string);
    // A!B!|
    let test = "AB|!";
    string = conjunctive_normal_form(test);
    println!("{}", &string);
    test_truth_table_eq(test, &string);
    // A!B!&
    let test = "AB|C&";
    string = conjunctive_normal_form(test);
    println!("{}", &string);
    test_truth_table_eq(test, &string);
    // AB|C&
    let test = "AB|C|D|EF||GHI|||";
    string = conjunctive_normal_form(test);
    println!("{}", &string);
    test_truth_table_eq(test, &string);
    // ABCD|||
    let test = "AB&C&D&";
    string = conjunctive_normal_form(test);
    println!("{}", &string);
    test_truth_table_eq(test, &string);
    // ABCD&&&
    let test = "AB&!C!|";
    string = conjunctive_normal_form(test);
    println!("{}", &string);
    test_truth_table_eq(test, &string);
    // A!B!C!||
    let test = "AB|!C!&";
    string = conjunctive_normal_form(test);
    println!("{}", &string);
    test_truth_table_eq(test, &string);
    // A!B!C!&&
    let test = "AB&CD&|";
    string = conjunctive_normal_form(test);
    println!("{}", &string);
    test_truth_table_eq(test, &string);
    let test = "AA^";
    string = conjunctive_normal_form(test);
    println!("{}", &string);
    test_truth_table_eq(test, &string);
}
