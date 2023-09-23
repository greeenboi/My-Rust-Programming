fn main() {
    let first_name = "suvan";
    
    println!("Hello, {}!", format!("{} {}", first_name, "102321"));

    let last_name = String::from("gs");
    println!("Hello, {}!", format!("{} {}", first_name, last_name));

}
