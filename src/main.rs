use java2rust::{generate_rust, parse_tree};

fn main() {
    let source_code = r#"
    class Test {
        int double(int x) {
            return x * 2;
        }
    }
    "#;
    let tree = parse_tree(source_code);
    println!("{:?}", tree);
    let mut rust_code = String::new();
    generate_rust(&tree, &mut rust_code);
    println!("=== Rust ===");
    println!("{rust_code}");
}
