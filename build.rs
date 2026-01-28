use std::{fs, path::Path};

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    copy_dir_all(
        &Path::new("assets"),
        &Path::new(format!("target/{}/assets", std::env::var("PROFILE").unwrap()).as_str()),
    )?;

    Ok(())
}
