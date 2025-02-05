use std::io;

struct TreeNode {
    value: i32,
    left: Option<usize>,
    right: Option<usize>,
}

struct BinaryTree {
    nodes: Vec<TreeNode>,
}

impl BinaryTree {
    fn new() -> Self {
        BinaryTree { nodes: vec![] }
    }

    fn insert(&mut self, value: i32) {
        let node = TreeNode {
            value,
            left: None,
            right: None,
        };
        if self.nodes.is_empty() {
            self.nodes.push(node);
        } else {
            let mut idx = 0;
            loop {
                if value < self.nodes[idx].value {
                    if let Some(left_idx) = self.nodes[idx].left {
                        idx = left_idx;
                    } else {
                        let new_idx = self.nodes.len();
                        self.nodes[idx].left = Some(new_idx);
                        self.nodes.push(node);
                        break;
                    }
                } else {
                    if let Some(right_idx) = self.nodes[idx].right {
                        idx = right_idx;
                    } else {
                        let new_idx = self.nodes.len();
                        self.nodes[idx].right = Some(new_idx);
                        self.nodes.push(node);
                        break;
                    }
                }
            }
        }
    }

    fn print_tree(&self) {
        self.print_subtree(0, 0);
    }

    fn print_subtree(&self, idx: usize, depth: usize) {
        if let Some(right_idx) = self.nodes[idx].right {
            self.print_subtree(right_idx, depth + 1);
        }
        println!("{:width$}{}", "", self.nodes[idx].value, width = depth * 4);
        if let Some(left_idx) = self.nodes[idx].left {
            self.print_subtree(left_idx, depth + 1);
        }
    }
}

pub fn main() {
    let mut binary_tree = BinaryTree::new();
    loop {
        println!("Digite um número para inserir na árvore (ou 'sair' para terminar):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "sair" {
            break;
        }
        if let Ok(num) = input.parse::<i32>() {
            binary_tree.insert(num);
            binary_tree.print_tree();
        } else {
            println!("Entrada inválida. Por favor, digite um número inteiro.");
        }
    }
}