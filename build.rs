use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("src") {
        let ent = entry.unwrap();
        let path = ent.path();
        if let Some(name) = path.file_name() {
            if let Some(s) = name.to_str() {
                if !s.starts_with("#") && !s.starts_with(".#") && s.ends_with(".rs") {
                    println!("cargo:rerun-if-changed={}", path.display());
                }
            }
        }
    }
    println!("cargo:rerun-if-changed=build.rs");
}
