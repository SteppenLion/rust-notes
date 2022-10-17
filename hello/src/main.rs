fn main() {
    // just comment
    /*
    Just a
    another
    multiline comment
    */
    println!("Hello, world!"); // will be on the new line

    // basic output comands with {} -> placeholder
    // numbers
    println!("Value of the constant is : {}", 10);
    // string values
    println!(
        "String values: My fist name is {} and my last name is {}",
        "Mark", "Kishon"
    );
    println!(
        "{3} can {2} {0} arguments {1}",
        "positional", "also", "use", "we"
    );
    // named arguments
    println!(
        "{language} is the future and number {position} in StackOverflow survey",
        language = "Rust",
        position = 1
    );
    // basic math
    println!("The sumation of 4 + 3 = {}", 4 + 3);
    print!("print! function will not put new line,  ");
    print!("only the println! function will");
    println!("modulo of 5 and 2 is: {}", 5 % 2);
}
