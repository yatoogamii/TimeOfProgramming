/* fn main() {
    let mut x = 5;
    
    println!("number is: {}", x);

    x = 10;

    println!("number is: {}", x);
} */

fn main() {
    const X: u32 = 5;
    
    fn test() {
        println!("number is: {}", X);
    }

    test();

    fn test2() {
        println!("number is: {}", X);
    }

    test2();
}