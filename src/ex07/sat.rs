use crate::ex03::evaluation::eval_formula;
use crate::ex06::conjunctiveform::conjunctive_normal_form;
use crate::utils::binary_tree::*;
const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

use num::pow;

fn get_bit(num: u32, index: u32) -> u32 {
    (num >> index) & 1
}

fn get_max(length: usize) -> u32 {
    pow(2, length)
}

fn test_mini_tree(mut tree: LogicalBT) -> bool {
    let formula = tree.get_string();
    let mut used_letters = String::new();
    for c in LETTERS.chars() {
        match formula.find(c) {
            Some(_) => {
                used_letters.push(c);
            }
            None => {}
        }
    }
    let mut data = vec![0; used_letters.len()];
    let reverse = used_letters.len() - 1;
    for x in 0..get_max(used_letters.len()) {
        let mut copy: String = formula.clone();
        for i in 0..used_letters.len() {
            data[reverse - i] = get_bit(x, i as u32);
            copy = copy.replace(
                used_letters.chars().nth(reverse - i).unwrap(),
                &data[reverse - i].to_string(),
            );
        }
        if eval_formula(&copy) {
            return true;
        }
    }
    false
}

fn cut_tree(head: Option<Box<TNode>>, mut ret: bool) -> bool {
    if ret != true {
        return false;
    }
    match head.clone() {
        Some(val) => match val.operation {
            Op::Conjuction => {
                match val.left {
                    Some(v) => match v.operation {
                        Op::Conjuction => {
                            if ret {
                                ret = cut_tree(Some(v.clone()), ret);
                            }
                        }
                        _ => {
                            if ret {
                                ret = test_mini_tree(LogicalBT::new(*head.clone().unwrap()));
                            }
                            return ret;
                        }
                    },
                    None => {}
                };
                match val.right {
                    Some(v) => match v.operation {
                        Op::Conjuction => {
                            if ret {
                                ret = cut_tree(Some(v.clone()), ret);
                            }
                        }
                        _ => {
                            if ret {
                                ret = test_mini_tree(LogicalBT::new(*head.clone().unwrap()));
                            }
                            return ret;
                        }
                    },
                    None => {}
                }
                if ret {
                    ret = test_mini_tree(LogicalBT::new(*head.clone().unwrap()));
                    return ret;
                }
            }
            Op::Disjunction => {
                ret = test_mini_tree(LogicalBT::new(*head.clone().unwrap()));
                return ret;
            }
            _ => {}
        },
        None => {}
    }
    ret
}

pub fn sat(formula: &str) -> bool {
    let cnf_formula = conjunctive_normal_form(formula);
    let mut ret = true;
    let tree = LogicalBT::fill_variable_tree(&cnf_formula);
    let head = tree.head.unwrap();
    // get all mini trees from last & node
    ret = cut_tree(Some(Box::new(head)), ret);
    ret
}
