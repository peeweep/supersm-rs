use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct AppOptions {
    #[clap(short = 'a', long = "add", conflicts_with = "delete")]
    add: Option<String>,

    #[clap(short = 'd', long = "delete", conflicts_with = "add")]
    delete: Option<String>,

    #[clap(short = 't', long = "target", default_value = "..")]
    target: String,
}

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
        let thisfile_path = std::path::Path::canonicalize(std::path::Path::new(&args[0])).unwrap();
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

fn list_files(directory: &String) -> Vec<String> {
    let mut files = Vec::new();
    let directory_path = std::path::Path::new(directory);

    fn list_files_recursive(
        directory: &std::path::Path,
        top_directory: &std::path::Path,
        files: &mut Vec<String>,
    ) {
        if let Ok(entries) = std::fs::read_dir(directory) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    files.push(path.to_string_lossy().into_owned());
                } else if path.is_dir() {
                    list_files_recursive(&path, top_directory, files);
                }
            }
        }
    }

    list_files_recursive(directory_path, directory_path, &mut files);

    files.iter_mut().for_each(|file| {
        if file.starts_with(directory) {
            *file = file.replacen(directory, "", 1);
        }
    });

    files
}

fn main() -> std::io::Result<()> {
    let app_options = AppOptions::parse();

    // delete target
    if let Some(delete_value) = &app_options.delete {
        let files = list_files(delete_value);
        for filename in files {
            let target_file_path = format!("{}{}", app_options.target, filename);
            let target_file = std::path::Path::new(&target_file_path);
            clean_targetfile(target_file.to_path_buf())?;
        }
        return Ok(());
    }

    // add target
    if let Some(add_value) = &app_options.add {
        let files = list_files(add_value);
        for filename in files {
            let origin_file_path = format!("{}/{}", add_value, filename);
            let origin_file =
                std::path::Path::canonicalize(std::path::Path::new(&origin_file_path))?;
            let target_file_path = format!("{}{}", app_options.target, filename);
            let target_file = std::path::Path::new(&target_file_path);

            if let Some(parent) = target_file.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }

            let _ = clean_targetfile(target_file.to_path_buf());

            std::os::unix::fs::symlink(origin_file.clone(), target_file)?;
            println!(
                "ln -sfv {} {}",
                origin_file.to_str().unwrap(),
                target_file.to_str().unwrap()
            );
        }
    }

    Ok(())
}
