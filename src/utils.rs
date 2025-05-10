use std::{fs, path::Path};

pub fn ensure_directory_exists(dir: &str) -> () {
  let path = Path::new(dir);
  if !path.exists() {
    match fs::create_dir(&path) {
      Ok(_) => println!("Created directory: {}", dir),
      Err(e) => println!("Error creating directory: {}", e),
    }
  }
}

pub fn filter_by_suffix(dir: &str, suffix: &str) {
  // walk through the directory
  if let Ok(entries) = fs::read_dir(dir) {
    for entry in entries {
      if let Ok(entry) = entry {
        if let Some(entry_suffix) = entry.path().extension().and_then(|s| s.to_str()) {
          if entry_suffix != suffix {
            if let Err(e) = fs::remove_file(entry.path()) {
              eprintln!("Error removing file: {}", e);
            }
          }
        }
      }
    }
  } else {
    eprintln!("Error reading directory {}", dir);
  }
}

pub fn wipe_out_ds_store(dir: &str) -> () {
  let path = format!("{}/{}", dir, ".DS_Store");
  let file_exists = fs::exists(&path).expect("File doesn't exists!");
  if file_exists {
    fs::remove_file(path).expect("Can't remove file as file doesn't exists");
  }
}
