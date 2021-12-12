use std::collections::VecDeque;
use std::fmt::Debug;

#[derive(Debug)]
struct Tree<V> {
    children: Vec<Tree<V>>,
    value: V
}

impl<V> Tree<V> where V: Debug {
    pub fn bfs(&self, f: impl Fn(&V)) {
        let mut q = VecDeque::new(); // https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html
        q.push_back(self);
        println!("first in the tree is ==> {:#?}", self.value);

        while let Some(t) = q.pop_front() {
            (f)(&t.value);
            for child in &t.children {
                q.push_back(child);
            }
        }


    }
}

fn main() {
    let t = Tree {
        children: vec![
            Tree {
                children: vec![
                    Tree { children: vec![], value: 5 },
                    Tree { children: vec![], value: 6 }
                ],
                value: 2
            },
            Tree { children: vec![], value: 3 },
            Tree { children: vec![], value: 4 },
        ],
        value: 1
    };
    t.bfs(|v| println!("{}", v));
}
