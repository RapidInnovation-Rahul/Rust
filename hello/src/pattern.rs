fn main(){
    let n = 5;
    for i in 1..n+1{
        for _j in 1..i+1{
            print!("*");
        }
        println!();
    }
    for i in 1..n+1{
        for _j in 1..n+1-i{
            print!(" ");
        }
        for _j in 1..i+1{
            print!("* ");
        }
        println!();
    }
}