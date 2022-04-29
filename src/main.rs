pub mod ex00;
pub mod ex01;
pub mod ex02;
pub mod ex03;
pub mod ex04;
pub mod ex05;
pub mod ex06;
pub mod ex07;
pub mod ex08;
pub mod ex09;
pub mod ex10;
pub mod ex11;
pub mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("You must put the exercice number as an arg (from 0 to 11) to run the tests");
    } else {
        let exercice: i32 = args[1]
            .trim()
            .parse()
            .expect("you must give a number between 0 and 11");
        match exercice {
            0 => ex00::test::ex00(),
            1 => ex01::test::ex01(),
            2 => ex02::test::ex02(),
            3 => ex03::test::ex03(),
            4 => ex04::test::ex04(),
            5 => ex05::test::ex05(),
            6 => ex06::test::ex06(),
            7 => ex07::test::ex07(),
            8 => ex08::test::ex08(),
            9 => ex09::test::ex09(),
            10 => ex10::test::ex10(),
            11 => ex11::test::ex11(),
            666 => {
                ex00::test::ex00();
                ex01::test::ex01();
                ex02::test::ex02();
                ex03::test::ex03();
                ex04::test::ex04();
                ex05::test::ex05();
                ex06::test::ex06();
                ex07::test::ex07();
                ex08::test::ex08();
                ex09::test::ex09();
                ex10::test::ex10();
                ex11::test::ex11();
            }
            _ => {
                println!("you must give a number between 0 and 11");
            }
        }
    }
}
