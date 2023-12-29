#![feature(absolute_path)]
fn clean_targetfile(path: std::path::PathBuf) {
    match std::fs::symlink_metadata(path.clone()) {
        Ok(_) => {
            println!("file exist, remove it");
            let _ = std::fs::remove_file(path.clone());
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                // Waiting do symlinking on main function.
                // println!("{:#?}", err);
            }
            _ => {
                println!("{:#?}", err);
            }
        },
    }

    // test case 1: file not exist
    // test case 2: file is symlink
    // test case 3: file is a bad symlink
    // test case 4: file exist but is a regular file, not a symlink file
}

fn main() -> std::io::Result<()> {
    // transfer target file path to a absolute file path
    // 1. remove old file
    let path = std::path::absolute("test/b.txt")?;
    clean_targetfile(path.clone());

    // 2. do a symlink
    println!("do a symlink");
    std::os::unix::fs::symlink("a.txt", path.clone())?;

    Ok(())
}
