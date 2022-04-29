use super::inverse_fn::reverse_map;

pub fn ex11() {
    let num: f64 = 0.0000008156057449094033;
    let res = reverse_map(num);
    println!("{} {}", res.0, res.1);
    let num: f64 = 0.;
    let res = reverse_map(num);
    println!("{} {}", res.0, res.1);
    let num: f64 = 0.25;
    let res = reverse_map(num);
    println!("{} {}", res.0, res.1);
    let num: f64 = 0.50;
    let res = reverse_map(num);
    println!("{} {}", res.0, res.1);
    let num: f64 = 0.75;
    let res = reverse_map(num);
    println!("{} {}", res.0, res.1);
    let num: f64 = 1.;
    let res = reverse_map(num);
    println!("{} {}", res.0, res.1);
}
