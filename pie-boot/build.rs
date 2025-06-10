use kdef_pgtable::*;
use std::{io::Write, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=link.ld");
    println!("cargo:rustc-link-search={}", out_dir().display());

    // let kimage_vaddr = KIMAGE_VADDR;
    let kimage_vaddr = 0x4020_0000;

    let mut ld = include_str!("link.ld").to_string();

    macro_rules! set_var {
        ($v:ident) => {
            ld = ld.replace(concat!("{", stringify!($v), "}"), &format!("{:#x}", $v));
        };
    }

    set_var!(kimage_vaddr);

    let mut file =
        std::fs::File::create(out_dir().join("pie_boot.x")).expect("pie_boot.x create failed");
    file.write_all(ld.as_bytes())
        .expect("pie_boot.x write failed");
}

fn out_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
}