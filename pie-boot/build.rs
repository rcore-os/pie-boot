use kdef_pgtable::*;
use std::{io::Write, path::PathBuf, process::Command};

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

    if std::env::var("TARGET").unwrap().contains("aarch64-") {
        aarch64_set_loader();
    }
}

fn out_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
}

fn aarch64_set_loader() {
    let loader_path =
        std::env::var_os("CARGO_BIN_FILE_PIE_BOOT_ARM_LOADER").expect("loader binary");
    let loader_dst = out_dir().join("loader.bin");

    let status = Command::new("rust-objcopy")
        .args(["--strip-all", "-O", "binary"])
        .arg(&loader_path)
        .arg(loader_dst)
        .status()
        .expect("objcopy failed");

    assert!(status.success());

    println!("target dir: {}", target_dir().display());

    let _ = std::fs::remove_file(target_dir().join("loader.elf"));
    std::fs::copy(&loader_path, target_dir().join("loader.elf")).unwrap();
}

fn target_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .ancestors()
        .nth(3)
        .unwrap()
        .into()
}
