#![feature(absolute_path)]
fn clean_targetfile(path: std::path::PathBuf) -> std::io::Result<()> {
    match std::fs::symlink_metadata(path.clone()) {
        Ok(_) => {
            println!("remove old file: {:?}", path.clone());
            std::fs::remove_file(path.clone())?;
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                return Ok(());
            }
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
        let path = std::path::absolute("file_not_found.txt").unwrap();
        let result = clean_targetfile(path);
        assert!(result.is_ok(), "Test case1 failed");
        println!("Test case1 passed");
    }

    // test case 2: file is symlink
    #[test]
    fn test_case2() {
        let args: Vec<String> = std::env::args().collect();
        let thisfile_path = std::path::absolute(&args[0]).unwrap();
        // let good_symlink_path = thisfile_path.parent().unwrap().join("good_symlink.txt");
        let good_symlink_path = std::path::absolute("/tmp/good_symlink.txt").unwrap();
        let _ = std::os::unix::fs::symlink(thisfile_path, good_symlink_path.clone());
        match clean_targetfile(good_symlink_path) {
            Ok(_) => {
                println!("Test case2 passed");
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }

    // test case 3: file is a bad symlink
    #[test]
    fn test_case3() {}

    // test case 4: file found but not a symlink file
    #[test]
    fn test_case4() {}
}

fn main() -> std::io::Result<()> {
    // 1. remove old file
    // let path = std::fs::canonicalize("test/b.txt")?;
    let path = std::path::absolute("test/b.txt")?;

    let _ = clean_targetfile(path.clone());

    // 2. do a symlink
    println!("do a symlink");
    std::os::unix::fs::symlink("a.txt", path.clone())?;

    Ok(())
}
