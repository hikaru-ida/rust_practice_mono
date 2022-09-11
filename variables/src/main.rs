fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of x in the inner scope is:{}", x);
    }
    print!("The value of x is: {}", x);
}
