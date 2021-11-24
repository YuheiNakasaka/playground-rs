use playground::merkle_tree::Tree;

#[warn(unreachable_patterns)]
fn main() {
    let data = vec!["a", "b", "c", "d"];
    let mut tree = Tree::new(data);
    tree.build_tree();

    let merkle_pass = tree.get_markle_pass("b".to_string());
    for pass in &merkle_pass {
        println!("Hash: {}, Pos: {}", pass[0], pass[1]);
    }

    let hash = tree.calc(merkle_pass);
    println!("Expected: {:?}", tree.root);
    println!("Output: {:?}", hash);
}
