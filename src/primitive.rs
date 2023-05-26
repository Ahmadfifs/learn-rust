
pub fn run(){
	let number: i32 = 10;
    let string: &str = "Ahmad";
    let charachter: char= 'a';
    let boolean: bool= true;
    let float: f64 = 0.1;

    println!("Hello, world! {number}");
    println!("Hello, world! {}", string);
    println!("Hello, world! {}", charachter);
    println!("Hello, world! {}", boolean);
    println!("Hello, world! {}", float);
    println!("Hello, world! {}", add(1, 2));
}

fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}