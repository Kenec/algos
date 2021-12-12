use std::vec;

struct Tree<V> {
    children: Vec<Tree<V>>,
    value: V
}

impl<V> Tree<V> {
    pub fn dfs<F>(&self, f: F) where F: Fn(&V) {
        self.dfs_helper(&f);
    }

    fn dfs_helper<F>(&self, f: &F) where F: Fn(&V) {
        (f)(&self.value);
        for child in &self.children {
            child.dfs_helper(f);
        }
    }
}

fn main() {
    let t: Tree<i32> = Tree {
        children: vec![
            Tree {
                children: vec![
                    Tree {
                        children: vec![],
                        value: 14
                    }
                ],
                value: 28
            },
            Tree {
                children: vec![],
                value: 80
            }
        ],
        value: 50
    };

    t.dfs(|node| { println!("{}", node); });
}
