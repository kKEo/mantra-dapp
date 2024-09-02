pub fn main() {
  let mut greeting = String::from("Hello");
  greeting.push_str(", Mantra");
  println!("{}", greeting);

  let mut my_slice: &str = "This is a string slice!";
  // my_slice.push_str("More text"); // Try to add this line, it will not work
  println!("{}", my_slice);
  my_slice = "Another content"; // but we can change a reference to the variable
  println!("{}", my_slice);

  // concatenation
    let first_name = String::from("Alice");
    let last_name = String::from("Johnson");
    let full_name = first_name + " " + &last_name;
    println!("{}", full_name); // Output: "Alice Johnson"

  // String indexing
    let message = "Hello, MANTRA Chain!";
    let first_char = message.chars().nth(0).unwrap(); // Get the first character 'H'
    println!("The first character is: {}", first_char); // Outputs: 'H'

    let last_char = message.chars().nth(message.chars().count() - 1).unwrap(); // Get the last character '!'
    println!("The last character is: {}", last_char); // Outputs: '!'

    // slicing
    let hello_slice = &message[0..5];       // Extract "Hello"
    println!("{}", hello_slice);          // Output: Hello

    let mantra_slice = &message[7..13];     // Extract "MANTRA"
    println!("{}", mantra_slice);           // Output: MANTRA

    let chain_slice = &message[14..];        // Extract "Chain!"
    println!("{}", chain_slice);            // Output: Chain!

    let whole_string_slice = &message[..]; // Extract the entire string
    println!("{}", whole_string_slice);   // Output: Hello, MANTRA Chain!
}
