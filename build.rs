use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("src") {
        println!("cargo:rerun-if-changed={}", entry.unwrap().path().display());
    }
    println!("cargo:rerun-if-changed=build.rs");
}
