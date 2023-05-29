use java2rust::parse_tree;

fn main() {
    let source_code = r#"
    class Test {
        int double(int x) {
            return x * 2;
        }
    }
    "#;
    let tree = parse_tree(source_code);
}
