use std::io;
fn main() {
    let mut num = String::new();
    let mut num2 = String::new();
    let mut num3 = String::new();
    loop {
        num3 = String::from("");
        num2 = String::from("");
        num = String::from("");

        println!("Enter a number A");
        match io::stdin().read_line(&mut num) {
            Ok(_) => {}
            Err(e) => {
                println!("Fatal error - {}", e);
            }
        }
        println!("Enter a number B");
        match io::stdin().read_line(&mut num2) {
            Ok(_) => {
            }
            Err(e) => {
                println!("Fatal errorа - {}", e);
            }
        }
        println!("Выбирайте 1 - +, 2 - -, 3 - *, 4 - /");
        match io::stdin().read_line(&mut num3) {
            Ok(_) => {}
            Err(e) => {
                println!("Fatal error - {}", e);
            }
        }
        let mut re = 0;
        re = num3.trim().to_owned().parse().unwrap();
        if re == 1 {
            let mut a: f64 = 0.0;
            a = num.trim().to_owned().parse().unwrap();
            let mut b: f64 = 0.0;
            b = num2.trim().to_owned().parse().unwrap();
            let r:f64 = b + a;
            println!("{}", r);
        } else if re == 2 {
            let mut a: f64 = 0.0;
            a = num.trim().to_owned().parse().unwrap();
            let mut b: f64 = 0.0;
            b = num2.trim().to_owned().parse().unwrap();
            let r:f64 = b - a;
            println!("{}", r);
        } else if re == 3 {
            let mut a: f64 = 0.0;
            a = num.trim().to_owned().parse().unwrap();
            let mut b: f64 = 0.0;
            b = num2.trim().to_owned().parse().unwrap();
            let r:f64 = b * a;
            println!("{}", r);
        } else if re == 4 {
            let mut a: f64 = 0.0;
            a = num.trim().to_owned().parse().unwrap();
            let mut b: f64 = 0.0;
            b = num2.trim().to_owned().parse().unwrap();
            let r:f64 = b / a;
            println!("{}", r);
        } else {
            println!("Fatal error")
        }
    }
}
