i#[derive(Debug)] // Add derive for Debug to allow printing
enum Priority {
    High,
    Medium,
    Low,
}

struct ToDoItem {
    task: String,
    priority: Priority, 
}

pub fn main() {
    // Create a to-do item with high priority
    let task1 = ToDoItem {
        task: String::from("Complete CosmWasm tutorial"),
        priority: Priority::High,
    };

    // Create a to-do item with medium priority
    let task2 = ToDoItem {
        task: String::from("Learn about MANTRA Chain"),
        priority: Priority::Medium,
    };

    // Create a to-do item with low priority to avoid the warning
    let task3 = ToDoItem {
        task: String::from("Take a break"),
        priority: Priority::Low,
    };

    // Print information about the tasks
    println!("Task 1: {}, Priority: {:?}", task1.task, task1.priority);
    println!("Task 2: {}, Priority: {:?}", task2.task, task2.priority);
    println!("Task 3: {}, Priority: {:?}", task3.task, task3.priority); 

    // Example of matching on the priority
    match task1.priority {
        Priority::High => println!("This task is urgent!"),
        Priority::Medium => println!("This task is important."),
        Priority::Low => println!("This task can wait."),
    }
}

