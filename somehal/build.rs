use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search={}", out_dir().display());

    let ld_name = "somehal";
    let ld_name_out = format!("{ld_name}.x");

    println!("cargo::rustc-link-arg=-T{ld_name_out}");

    let script = format!("{ld_name}.ld");

    println!("cargo:rerun-if-changed={script}");
    let ld_content = std::fs::read_to_string(script).unwrap();

    std::fs::write(out_dir().join(ld_name_out), ld_content).expect("link.x write failed");
}

fn out_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
}
