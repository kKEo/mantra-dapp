pub fn main() {
    let num_tasks: u32 = 10;         // Unsigned 32-bit integer for task count
    let my_token_balance: u64 = 100000000;  // Unsigned 128-bit integer for token balance
    let profit_or_loss: i64 = -500;  // Signed 64-bit integer for profit/loss
    let default_int: i32 = -5;       // Signed 32-bit integer (i32 is inferred)

    // Print the values to the console:
    println!("Number of tasks: {}", num_tasks);           
    println!("My token balance: {}", my_token_balance); 
    println!("Profit/loss: {}", profit_or_loss);         
    println!("Default integer: {}", default_int);  
}
