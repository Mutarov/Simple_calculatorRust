use std::io;


fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    loop {
    println!("<=:\"/\\------------------------------------------------------/\\\":=>");
    input(&mut a, &mut b, &mut c);
    calculate(a.trim().parse().unwrap(), b.trim().parse().unwrap(), c.trim().parse().unwrap());
    }
}
fn calculate(n1: i64, n2: i64, n3:isize) {
    let result = 0;
    match n3 {
        1 => { println!("{} + {} = {}", n1, n2, n1 + n2) },
        2 => println!("{} - {} = {}",n1,n2,n1 - n2),
        3 => println!("{} * {} = {}",n1,n2,n1 * n2),
        4 => println!("{} / {} = {}",n1,n2,n1 / n2),
        //5 => println!("{:.0}",(n1 as f64 + n2 as f64) / 2.0),
        _ => println!("ERROR")
    }

}
fn input(n1: &mut String, n2: &mut String, n3: &mut String) {
    println!("Number A");
    io::stdin().read_line(n1).expect("ERROR");
    println!("Number B");
    io::stdin().read_line(n2).expect("ERROR");
    println!("1 - +, 2 - -, 3 - *,4 - /, 5 - Average(Not working!)");
    io::stdin().read_line(n3).expect("ERROR");
    println!("<=:\"/\\------------------------------------------------------/\\\":=>");
}