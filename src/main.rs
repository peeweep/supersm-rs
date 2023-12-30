#![feature(absolute_path)]
fn clean_targetfile(path: std::path::PathBuf) -> std::io::Result<()> {
    match std::fs::symlink_metadata(&path) {
        Ok(_) => {
            println!("remove old file: {:?}", &path);
            std::fs::remove_file(&path)?;
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {}
            _ => {
                return Err(err);
            }
        },
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::clean_targetfile;

    // test case 1: file not found
    #[test]
    fn test_case1() {
        let path = std::env::temp_dir().join("file_not_found.txt");
        assert!(clean_targetfile(path).is_ok(), "Test case1 failed");
    }

    // test case 2: file is a symlink
    #[test]
    fn test_case2() {
        let args: Vec<String> = std::env::args().collect();
        let thisfile_path = std::path::absolute(&args[0]).unwrap();
        let good_symlink_path = std::env::temp_dir().join("good_symlink.txt");
        let _ = std::os::unix::fs::symlink(thisfile_path, &good_symlink_path);
        assert!(
            clean_targetfile(good_symlink_path).is_ok(),
            "Test case2 failed"
        );
    }

    // test case 3: file is a bad symlink
    #[test]
    fn test_case3() {
        let bad_symlink_path = std::env::temp_dir().join("bad_symlink.txt");
        let _ = std::os::unix::fs::symlink(
            std::env::temp_dir().join("filenotexist"),
            &bad_symlink_path,
        );
        assert!(
            clean_targetfile(bad_symlink_path).is_ok(),
            "Test case3 failed"
        );
    }

    // test case 4: file is a real file
    #[test]
    fn test_case4() {
        let real_file = std::env::temp_dir().join("real_file.txt");
        std::fs::write(&real_file, "Hello, World!").expect("Failed to create real file");
        assert!(clean_targetfile(real_file).is_ok(), "Test case4 failed");
    }
}
mod options;
fn main() -> std::io::Result<()> {
    let app_options = options::AppOptions::new();

    println!("target: {}", app_options.target);
    if app_options.delete != "" {
        println!("delete: {}", app_options.delete);
        return Ok(());
    }
    println!("add: {}", app_options.add);

    Ok(())
}
