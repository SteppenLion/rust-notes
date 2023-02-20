fn main() {
    println!("Hello, world!");
    println!(
        "Hello there {name} {last_name}",
        name = "Marek",
        last_name = "Kishon"
    );
    fn name(first_name: &String, last_name: &String, birthday: &String) {
        println!(
            "Hello {0} {1}, your birthday is {2}",
            first_name, last_name, birthday
        );
    }
    name("Marek", "Someone", "1.9.1939")
}
