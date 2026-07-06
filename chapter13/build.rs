use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let src_assets = Path::new(&manifest_dir).join("src").join("assets");
    let dst_assets = Path::new(&manifest_dir)
        .join("target")
        .join(&profile)
        .join("assets");

    if src_assets.exists() {
        if dst_assets.exists() {
            let _ = fs::remove_dir_all(&dst_assets);
        }
        let _ = copy_dir_all(&src_assets, &dst_assets);
    }

    println!("cargo:rerun-if-changed=src/assets");
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else {
            fs::copy(entry.path(), &dst_path)?;
        }
    }
    Ok(())
}