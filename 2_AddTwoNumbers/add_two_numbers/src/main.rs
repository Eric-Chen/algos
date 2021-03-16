
#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let b = Some(Box::new(Node{
        val: 10,
        next: None
    }));

    if let None = b {
        println!("how can this happen!")
    } else {
        println!("I know it!")
    }
}
