fn main() {
    let first_name = "suvan";
    let name: &str = first_name;
    
    println!("Hello, {}!", format!("{} {}", name, "102321"));

    let last_name = String::from("gs");
    println!("Hello, {}!", format!("{} {}", name, last_name));

}
