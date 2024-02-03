#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0,0,0);
    let origin = dbg!(Point(0,0,0));

    println!("origin is {:?}", origin);
    println!("black is {:?}", black);

    println!("origin is {:?}", origin);
}
