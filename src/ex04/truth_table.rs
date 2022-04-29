const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
use crate::ex03::evaluation::eval_formula;
use num::pow;

fn get_bit(num: u32, index: u32) -> u32 {
    (num >> index) & 1
}

fn get_max(length: usize) -> u32 {
    pow(2, length)
}

fn get_truth_table(formula: &str) -> String {
    let mut used_letters = String::new();
    let mut output = String::new();
    for c in LETTERS.chars() {
        match formula.find(c) {
            Some(_) => {
                used_letters.push(c);
                output += &format!("| {} ", c);
            }
            None => {}
        }
    }
    if used_letters.is_empty() {
        panic!("no numbers to try in this formula")
    }
    // filling output
    output += &format!("| = |\n|---");
    for _ in 0..used_letters.len() {
        output += "|---"
    }
    output += "|\n";
    //
    let mut data = vec![0; used_letters.len()];
    let reverse = used_letters.len() - 1;
    for x in 0..get_max(used_letters.len()) {
        let mut copy: String = formula.into();
        for i in 0..used_letters.len() {
            data[reverse - i] = get_bit(x, i as u32);
            copy = copy.replace(
                used_letters.chars().nth(reverse - i).unwrap(),
                &data[reverse - i].to_string(),
            );
        }
        let ret: u32 = eval_formula(&copy) as u32;
        for i in 0..used_letters.len() {
            output += &format!("| {} ", data[i]);
        }
        output += &format!("| {} |\n", ret);
    }
    output
}
pub fn print_truth_table(formula: &str) {
    print!("{}", get_truth_table(formula));
}

pub fn test_truth_table_eq(formula1: &str, formula2: &str) {
    assert_eq!(get_truth_table(formula1), get_truth_table(formula2))
}
