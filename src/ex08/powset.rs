use num::pow;
use std::collections::HashSet;

fn get_bit(num: u32, index: u32) -> u32 {
    (num >> index) & 1
}

fn get_max(length: usize) -> u32 {
    pow(2, length)
}

pub fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = Vec::new();
    let diff: Vec<i32> = set
        .into_iter()
        .map(|num| *num)
        .collect::<HashSet<i32>>()
        .into_iter()
        .collect();
    for n in 0..get_max(diff.len()) {
        ret.push(Vec::<i32>::new());
        for i in 0..diff.len() {
            let put = get_bit(n, i as u32);
            if put == 1 {
                ret[n as usize].push(diff[i])
            }
        }
    }
    ret
}
