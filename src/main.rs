mod utilities;

pub use utilities::stack::*;


fn main() {
    let mut stack = Stack::new();

    stack.push(11);
    stack.push(22);
    stack.push(33);

    let top = stack.pop();

    println!("{:?}", top.unwrap());
}
