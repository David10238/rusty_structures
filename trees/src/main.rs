mod binary_search_tree;

use binary_search_tree::BST;

fn main() {
    let mut tree: BST<i32> = BST::new();
    tree.add(24);
    tree.add(1);
    tree.add(-5);
    tree.add(-4);
    tree.add(680);
    tree.add(5000);
    tree.add(4900);
    tree.for_each();
    println!("Hello, trees!");
}
