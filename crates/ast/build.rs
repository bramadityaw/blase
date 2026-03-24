use type_sitter_gen::generate_nodes;

fn main() {
    println!("cargo::rerun-if-changed=../tree-sitter-blade");

    [
        ("src/blade.rs", tree_sitter_blade::NODE_TYPES),
        ("src/php.rs", tree_sitter_php::PHP_NODE_TYPES),
    ]
    .into_iter()
    .for_each(|(path, types)| {
        let path = std::env::current_dir().unwrap().join(path);

        let generated = generate_nodes(types).unwrap();

        std::fs::write(path, generated.into_string()).unwrap();
    });
}
