use hex;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Hash {
    value: String,
}

impl Hash {
    fn new(hash: String) -> Hash {
        Hash { value: hash }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    parent: Option<Box<Node>>,
    sibling: Option<Box<Node>>,
    position: String,
    data: String,
    hash: Hash,
}

#[derive(Debug)]
pub struct Tree {
    pub layers: Vec<Node>,
    pub root: Hash,
}

impl Tree {
    pub fn new(data: Vec<&str>) -> Tree {
        let mut layers: Vec<Node> = Vec::new();
        for datum in data.clone() {
            layers.push(Node {
                left: None,
                right: None,
                parent: None,
                sibling: None,
                position: "".to_string(),
                data: datum.to_string(),
                hash: gen_hash(datum.to_string()),
            });
        }
        Tree {
            layers: layers,
            root: Hash {
                value: "".to_string(),
            },
        }
    }

    pub fn build_tree(&mut self) {
        let mut layers: Vec<Node> = self.layers.clone();
        loop {
            if layers.len() <= 1 {
                break;
            }
            layers = self.build_layers(layers.clone());
        }
        self.layers = layers.clone();
        self.root = layers[0].hash.clone();
    }

    fn build_layers(&mut self, layers: Vec<Node>) -> Vec<Node> {
        let mut new_layers = vec![];
        for i in (0..(layers.len())).step_by(2) {
            let left = &mut layers[i].clone();
            let right = &mut layers[i + 1].clone();

            left.position = "left".to_string();
            right.position = "right".to_string();
            left.sibling = Some(Box::new(right.clone()));
            right.sibling = Some(Box::new(left.clone()));

            let data = format!("{}{}", left.hash.value, right.hash.value);
            let hash = gen_hash(data.clone());
            let mut parent = Node {
                left: Some(Box::new(left.clone())),
                right: Some(Box::new(right.clone())),
                parent: None,
                sibling: None,
                position: "".to_string(),
                data: data,
                hash: hash,
            };

            left.parent = Some(Box::new(parent.clone()));
            right.parent = Some(Box::new(parent.clone()));

            parent.left = Some(Box::new(left.clone()));
            parent.right = Some(Box::new(right.clone()));

            new_layers.push(parent);
        }
        new_layers
    }

    fn search(&self, input: String) -> Option<Box<Node>> {
        dfs(Some(Box::new(self.layers[0].clone())), input)
    }

    pub fn get_markle_pass(&self, input: String) -> Vec<Vec<String>> {
        let mut markle_pass: Vec<Vec<String>> = vec![];
        let node = self.search(input).unwrap();
        markle_pass.push(vec![
            node.left.as_ref().unwrap().hash.value.clone(),
            node.left.as_ref().unwrap().position.clone(),
        ]);
        markle_pass.push(vec![
            node.right.as_ref().unwrap().hash.value.clone(),
            node.right.as_ref().unwrap().position.clone(),
        ]);
        // markle_pass.push(vec![node.hash.value, node.position]);
        markle_pass.push(vec![
            node.sibling.as_ref().unwrap().hash.value.clone(),
            node.sibling.as_ref().unwrap().position.clone(),
        ]);
        return markle_pass;
    }

    pub fn calc(&self, merkle_pass: Vec<Vec<String>>) -> Hash {
        let mut ret = Hash {
            value: merkle_pass[0][0].clone(),
        };
        for i in 1..merkle_pass.clone().len() {
            let node = &merkle_pass[i];
            let val = &node[0];
            let pos = &node[1];
            if pos == "left" {
                ret = gen_hash(format!("{}{}", val, ret.value));
            } else {
                ret = gen_hash(format!("{}{}", ret.value, val));
            }
        }
        ret
    }
}

/// Return the parent node of the node of the target data.
fn dfs(node: Option<Box<Node>>, target: String) -> Option<Box<Node>> {
    match node {
        Some(obj) => {
            if obj.left.is_none() && obj.right.is_none() {
                return None;
            }
            let l_ref = obj.left.as_ref().unwrap();
            let r_ref = obj.right.as_ref().unwrap();
            if l_ref.data == target {
                return Some(obj);
            } else if r_ref.data == target {
                return Some(obj);
            } else {
                let l_ret = dfs(obj.left, target.clone());
                let r_ret = dfs(obj.right, target.clone());
                if l_ret.is_some() {
                    return l_ret;
                } else if r_ret.is_some() {
                    return r_ret;
                } else {
                    return None;
                }
            }
        }
        None => {
            return None;
        }
    }
}

/// Generate a SHA256 hash of the input string
fn gen_hash(data: String) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    Hash::new(hex::encode(hasher.clone().finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_hash() {
        let x = "hello".to_string();
        let ret = gen_hash(x);
        assert_eq!(
            ret.value,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_dfs() {
        //      root
        //     /   \
        //  p1       p2
        //  / \     / \
        // a   b   c   d
        let mut tree = Tree::new(vec!["a", "b", "c", "d"]);
        tree.build_tree();
        let p1 = dfs(Some(Box::new(tree.layers[0].clone())), "b".to_string());
        assert_eq!(p1.unwrap().right.unwrap().data, "b");
    }
}
