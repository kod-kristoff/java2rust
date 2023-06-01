mod options;
use clap::Parser;
use java2rust::{generate_rust, parse_tree, translator::translate_java};
use options::Args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    translate_java(&args.source)?;
    // let source_code = r#"
    // class Test {
    //     int double(int x) {
    //         return x * 2;
    //     }
    // }
    // "#;
    // let tree = parse_tree(source_code);
    // println!("{:?}", tree);
    // let mut rust_code = String::new();
    // generate_rust(&tree, &mut rust_code);
    // println!("=== Rust ===");
    // println!("{rust_code}");
    Ok(())
}
