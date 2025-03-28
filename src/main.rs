use std::collections::{VecDeque, HashMap};

fn main() {

    // Stack implemented using VecDeque's front operations
    println!("{}", "Stack proof:");
    // Create new stack
    let mut stack: VecDeque<u32> = VecDeque::new();

    // Basic stack operations
    // Push operation
    stack.push_front(1);
    stack.push_front(2);
    stack.push_front(3);
    print!("{}", "Push (initial stack): ");
    println!("{:?}", stack);

    // Top operation
    print!("{}", "Top: ");
    println!("{:?}", stack.front().unwrap());

    // Pop operation
    stack.pop_front();
    print!("{}", "Pop: ");
    println!("{:?}", stack);

    // isEmpty operation
    print!("{}", "isEmpty: ");
    println!("{:?}", stack.is_empty());



    // Queue implemented using VecDeque's both front and back operations
    println!("{}", "");
    println!("{}", "Queue proof:");
    // Basic queue operations
    // Create new queue
    let mut queue: VecDeque<u32> = VecDeque::new();

    // Enqueue operation
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);
    print!("{}", "Enqueue (initial queue): ");
    println!("{:?}", queue);

    // Peek/Front operation
    print!("{}", "Peek/Front: ");
    println!("{:?}", queue.front().unwrap());

    // Size operation
    print!("{}", "Size: ");
    println!("{:?}", queue.len());

    // Dequeue operation
    queue.pop_front();
    print!("{}", "Dequeue: ");
    println!("{:?}", queue);

    // isEmpty operation
    print!("{}", "isEmpty: ");
    println!("{:?}", queue.is_empty());

    // Hashmap available in Rust's standard library

    println!("{}", "");
    println!("{}", "Hashmap proof:");

    // Create new hashmap
    let mut hashmap = HashMap::new();

    // Basic hashmap operations

    // Insert operation

    hashmap.insert("Key1", 1);
    hashmap.insert("Key2", 2);
    hashmap.insert("Key3", 3);
    println!("{}", "Insert (initial hashmap): ");
    // & is used to not take ownership of the hashmap and be able to use it later
    for (key, value) in hashmap.iter() {
        println!("{}: {}", key, value)
    }

    // Get operation
    print!("{}", "Get Key1: ");
    println!("{:?}", hashmap.get(&"Key1").unwrap());

    // Remove operation
    println!("{}", "Remove Key1: ");
    hashmap.remove(&"Key1");
    for (key, value) in hashmap.iter() {
        println!("{}: {}", key, value)
    }

}
