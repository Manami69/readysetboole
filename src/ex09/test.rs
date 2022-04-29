use super::eval_set::eval_set;

pub fn ex09() {
    let sets = [[0, 1, 2], [0, 3, 4]];

    let result = eval_set("AB&", &sets);
    println!("{:?}", result);
    // [0]
    let sets = [[0, 1, 2], [3, 4, 5]];
    let result = eval_set("AB|", &sets);
    println!("{:?}", result);
    // [0, 1, 2, 3, 4, 5]
    let sets = [[0, 1, 2]];
    let result = eval_set("A!", &sets);
    println!("{:?}", result);
    // []
    let sets = [[4, 3, 2], [1, 0, 3]];
    let result = eval_set("A!B&", &sets);
    println!("{:?}", result);
    let sets = [[4, 3, 2], [1, 0, 3]];
    let result = eval_set("A!B^", &sets);
    println!("{:?}", result);
}
