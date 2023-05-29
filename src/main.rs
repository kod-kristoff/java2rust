use tree_sitter::{Language, Node, Parser};
use tree_sitter_traversal::{traverse, traverse_tree, Order};
extern "C" {
    fn tree_sitter_java() -> Language;
}

fn main() {
    println!("Hello, world!");
    let code = r#"
    class Test {
        int double(int x) {
            return x * 2;
        }
    }"#;
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_java() };
    dbg!(&language);
    parser
        .set_language(language)
        .expect("Error loading Java grammar");
    let parsed = parser.parse(code, None);
    let tree = parsed.expect("a valid tree");
    println!("{:#?}", tree.root_node().to_sexp());

    // let root_node = tree.root_node();
    // let mut walker = tree.walk();
    // // println!("{}: {:?}", walker.node().kind(), walker.field_name());
    // // loop {
    // //     println!("{}: {:?}", walker.node().kind(), walker.field_name());
    // //     if !walker.goto_next_sibling() {
    // //         break;
    // //     }
    // //     if !walker.goto_first_child() {
    // //         break;
    // //     }
    // // }
    // print_node(&root_node);
    // print_children(&root_node, 0);
    // for child_i in 0..root_node.child_count() {
    //     let child_node = root_node.child(child_i).unwrap();
    // }
    let preorder: Vec<Node<'_>> = traverse(tree.walk(), Order::Pre).collect::<Vec<_>>();
    dbg!(&preorder);
    let mut rust_code = String::new();
    let mut java_ast = JavaFile::new();
    for node in &preorder {
        print_node(code, node);
        java_ast.process_node(code, node);
        // translate_node(code, node, &mut rust_code);
    }
    println!("{:#?}", java_ast);
    println!("{}", rust_code);
}

fn print_node(source: &str, node: &Node) {
    println!(
        "{}: {}",
        node.kind(),
        &source[node.start_byte()..node.end_byte()]
    );
}

fn translate_node(source: &str, node: &Node, dst: &mut String) {
    match node.kind() {
        "class" => dst.push_str("struct"),
        "identifier" => dst.push_str(&format!(" {}", &source[node.start_byte()..node.end_byte()])),
        "{" => dst.push_str("{"),
        kind => println!("skipping {}", kind),
    }
}

fn print_children(node: &Node, level: usize) {
    for child_i in 0..node.child_count() {
        println!(
            "{:?}",
            node.field_name_for_child(child_i.try_into().unwrap())
        );
    }
}

#[derive(Debug, Default)]
pub struct JavaFile {
    curr_class: Option<Class>,
    class_decl: Option<ClassDecl>,
}

#[derive(Debug, Default)]
pub struct Class {
    decl: ClassDecl,
    body: ClassBody,
}

#[derive(Debug, Default)]
pub struct ClassDecl {
    name: Option<String>,
}

#[derive(Debug, Default)]
pub struct ClassBody {
    curr_method: Option<Method>,
    method_decl: Option<MethodDecl>,
}

#[derive(Debug, Default)]
pub struct Method {
    decl: MethodDecl,
    body: MethodBody,
}

#[derive(Debug, Default)]
pub struct MethodDecl {
    name: Option<String>,
}

#[derive(Debug, Default)]
pub struct MethodBody {
    // curr_method: Option<Method>,
}
impl Class {
    pub fn new(decl: ClassDecl) -> Self {
        Self {
            decl,
            body: ClassBody::default(),
        }
    }

    fn method_decl(&mut self) {
        self.body.method_decl = Some(MethodDecl::default())
    }
    fn add_identifier(&mut self, ident: &str) {
        self.body.add_identifier(ident);
    }
}

impl ClassBody {
    // pub fn new(decl: ClassDecl) -> Self {
    //     Self {
    //         decl,
    //         body: ClassBody::default(),
    //     }
    // }

    // fn method_decl(&mut self) {
    //     self.body.curr_method = Some(Method::default())
    // }
    fn add_identifier(&mut self, ident: &str) {
        if self.method_decl.is_some() {
            self.method_decl.as_mut().unwrap().name = Some(ident.to_string());
        }
    }
}
impl JavaFile {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_node(&mut self, source: &str, node: &Node) {
        // println!("node kind: {}", node.kind());
        match node.kind() {
            "class_declaration" => self.class_decl(),
            "class_body" => self.class_body(),
            "identifier" => self.add_identifier(&source[node.start_byte()..node.end_byte()]),
            "method_declaration" => self.method_decl(),
            kind => println!("skipping node kind '{}'", kind),
        }
    }

    fn open_class(&mut self) {
        self.curr_class = Some(Class::default())
    }

    fn class_body(&mut self) {
        self.curr_class = Some(Class::new(self.class_decl.take().unwrap()));
    }

    fn class_decl(&mut self) {
        self.class_decl = Some(ClassDecl::default())
    }

    fn method_decl(&mut self) {
        self.curr_class
            .as_mut()
            // .expect("curr class")
            .map(|c| c.method_decl());
    }
    fn add_identifier(&mut self, ident: &str) {
        if self.class_decl.is_some() {
            self.class_decl.as_mut().unwrap().name = Some(ident.to_string());
        } else if self.curr_class.is_some() {
            self.curr_class
                .as_mut()
                .expect("curr class")
                .add_identifier(ident);
        }
    }
}
