use merkle_tree::Tree;

fn main() {
    let data = vec!["a", "b", "c", "d"];
    let tree = Tree::new(data);
    println!("{:?}", tree.root);
}
