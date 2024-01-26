use avl_tree::AVLTree;

fn main() {
    let mut tree = AVLTree::new(0);

    for i in 1..10000000 {
        tree.add_value(i);
    }

    println!("{} elements", tree.get_values().last().unwrap() + 1);
    println!("Height: {}", tree.height());
}
