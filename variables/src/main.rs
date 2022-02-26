fn main() {
    let x = 5;
    let x = x + 5;
    {
        let x = 15;
        println!("The value is : {}",x);
    }
}