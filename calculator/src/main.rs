fn main() {
    let name: &str = "∞";
    println!(" {} vs {}", name.len() , name.chars().count());
    let answer = myfunc(1, 2);

    println!("1 / 2 = {}", answer);
}
fn myfunc(x: i64, y: u8) -> i64 {
    return x / (y as i64);
}
    
