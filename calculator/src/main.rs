fn main() {
    let first_name = "suvan";
    let name = String::from(first_name);
    
    // println!("Hello, {}!", format!("{} {}", first_name, "102321"));

    let last_name = String::from("gs");
    let new_name = name.to_owned() + " " + &last_name;
    // println!("Hello, {}!", format!("{} {}", name, last_name));

    println!("Hello bro, {}!", format!("{}",  new_name));

}
