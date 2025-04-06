// TODO: Update the function argument type
pub fn print_message(message: &String) {
    println!("Message: {}", message);
}

// Example Usage
pub fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    print_message(&gift_message);

    println!("{}", gift_message);
}
