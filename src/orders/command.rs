use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Command {
    time: u64, // Declared time
    // Other command fields
}



// fn main() {
//     let mut queue: BinaryHeap<Reverse<Command>> = BinaryHeap::new();

//     // Example commands with different times
//     queue.push(Reverse(Command { time: 10, payload: "First Command".into() }));
//     queue.push(Reverse(Command { time: 5, payload: "Second Command".into() }));
//     queue.push(Reverse(Command { time: 20, payload: "Third Command".into() }));

//     // Process commands in order
//     while let Some(Reverse(command)) = queue.pop() {
//         // Process command
//         println!("Processing command: {:?}", command);
//     }
// }