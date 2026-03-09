use type_sitter_gen::generate_nodes;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let out_path = std::env::current_dir().unwrap().join("src/nodes.rs");

    println!("cargo::rerun-if-changed=../tree-sitter-blade");

    let generated = generate_nodes(tree_sitter_blade::NODE_TYPES).unwrap();

    std::fs::write(out_path, generated.into_string()).unwrap();
}
