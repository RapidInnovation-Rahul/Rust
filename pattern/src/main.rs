use std::io;
fn main() {
    
    let mut num = String::new();
    io::stdin().read_line(& mut num).expect("failed to read!!!");
    
    let n: u32 = num.trim().parse().expect("failed to parse");
    println!("{}", n);
    for i in 1..=n{
        for _j in 0..(n-i){
            print!(" ");
        }
        for _j in 0..i{
            print!("* ");
        }
        println!();
    }
}