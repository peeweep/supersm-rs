#![feature(absolute_path)]
fn clean_targetfile(path: std::path::PathBuf) -> std::io::Result<()> {
    match std::fs::symlink_metadata(&path) {
        Ok(_) => {
            println!("rm -v {}", path.clone().to_str().unwrap());
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

fn list_files(directory: &str) -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_str) = file_name.to_str() {
                            // println!("{}/{}", directory, file_str.to_string());
                            files.push(file_str.to_string());
                        }
                    }
                }
            }
        }
    }

    files
}

fn main() -> std::io::Result<()> {
    let app_options = options::AppOptions::new();

    // delete target
    if app_options.delete != "" {
        let files = list_files(&app_options.delete.clone());
        for filename in files {
            let target_file =
                std::path::absolute(app_options.target.clone() + "/" + &filename).unwrap();
            clean_targetfile(target_file.clone())?;
        }
        return Ok(());
    }

    // add target
    let files = list_files(&app_options.add.clone());
    for filename in files {
        let origin_file = std::path::absolute(app_options.add.clone() + "/" + &filename).unwrap();
        let target_file =
            std::path::absolute(app_options.target.clone() + "/" + &filename).unwrap();
        let _ = clean_targetfile(target_file.clone());

        std::os::unix::fs::symlink(origin_file.clone(), target_file.clone())?;
        println!(
            "ln -sfv {} {}",
            origin_file.clone().to_str().unwrap(),
            target_file.clone().to_str().unwrap()
        );
    }

    Ok(())
}
