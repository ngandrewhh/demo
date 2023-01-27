use std::{fmt::{Debug, Display}};

#[derive(Debug)]
struct LinkedNode<T> {
    data: T,
    next: Option<Box<LinkedNode<T>>>
}

impl<T: Debug + Display> LinkedNode<T> {
    fn new(data: T, next: Option<Box<LinkedNode<T>>>) -> Self {
        LinkedNode { data: data, next: next }
    }

    fn add(&mut self, data: T) {
        // base case
        // (*self).next = Some(Box::new(LinkedNode { data, next: None }));

        let mut p = &mut (*self).next;
        
        loop {
            let q = match p {
                None => break,
                Some(U) => {
                    &mut *(U)
                },
            };

            p = &mut (*q).next;
        }
        
        (*p) = Some(Box::new(LinkedNode { data: data, next: None }));
    }

    fn print(&self) {
        let mut vec: Vec<&T> = vec![];
        vec.push(&self.data);

        let mut p = &self.next;
        
        loop {
            let q = match p {
                None => break,
                Some(U) => {
                    // let q = self.next.unwrap();
                    println!("from loop: {:?}", &(*U).data);
                    vec.push(&(*U).data);
                    &(*(U))
                },
            };

            p = &(*q).next;
        }

        println!("\n{}", vec.iter().map(ToString::to_string).collect::<Vec<String>>().join(" -> "));
    }
}

fn main() {
    println!("[[chain construction method]]");
    let head = 
    LinkedNode::new("test", 
        Some(Box::new(LinkedNode::new("is", 
            Some(Box::new(LinkedNode::new("working", 
                Some(Box::new(LinkedNode::new("!", None))))))))));

    head.print();

    println!("\n[[add method]]");
    let mut head_alt = LinkedNode::new(1.5, None);
    head_alt.add(2.5);
    head_alt.add(0.);
    head_alt.print();
}
