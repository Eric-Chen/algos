
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

    match b {
        Some(d) => {println!("{:?}", d);}
        _ => {println!("None")}
    }
}
