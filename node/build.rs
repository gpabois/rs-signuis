use std::{env, path::Path};

fn main() {
    node_bindgen::build::configure();
    copy_node_files();
}

fn copy_node_files() {
    let package_json_ipt = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("package.json");
    let package_json_opt = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("dist/package.json");
    std::fs::copy(package_json_ipt, package_json_opt).unwrap();

    let package_json_ipt = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("index.d.ts");
    let package_json_opt = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("dist/index.d.ts");
    std::fs::copy(package_json_ipt, package_json_opt).unwrap();
}