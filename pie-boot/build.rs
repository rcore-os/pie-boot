use std::{io::Write, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=link.ld");
    println!("cargo:rustc-link-search={}", out_dir().display());

    let ld = format!(include_str!("link.ld"),);

    let mut file =
        std::fs::File::create(out_dir().join("pie_boot.x")).expect("pie_boot.x create failed");
    file.write_all(ld.as_bytes())
        .expect("pie_boot.x write failed");
}

fn out_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
}
