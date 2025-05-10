use std::{fs, fs::File, io::Write};
use ncmdump::Ncmdump;
use crate::utils::{
    ensure_directory_exists,
    wipe_out_ds_store,
    filter_by_suffix
};

pub fn process_file(ncm_dir: &str, out_dir: &str, file_name: &str) -> std::io::Result<String> {
    let no_suffix_file_name = file_name.strip_suffix(".ncm").unwrap();
    let mp3_file_name = format!("{}.mp3", no_suffix_file_name);
    let mut ncm_dir = ncm_dir;
    let mut out_dir = out_dir;
    if ncm_dir.ends_with("/") {
        ncm_dir = ncm_dir.strip_suffix("/").unwrap();
    }
    if out_dir.ends_with("/") {
        out_dir = out_dir.strip_suffix("/").unwrap();
    }
    let ncm_file_path = format!("{}/{}", ncm_dir, file_name);
    let out_file_path = format!("{}/{}", out_dir, mp3_file_name);
    let out_file_path_clone = String::from(&out_file_path);
    let ncm_file = File::open(ncm_file_path.as_str())?;
    let mut ncm = Ncmdump::from_reader(ncm_file).expect("Can't create dump");
    let music = ncm.get_data().expect("Can't get data");
    
    let mut target = File::options()
         .create(true)
         .write(true)
         .open(out_file_path.as_str())?;

    if let Ok(_) = target.write_all(&music){
        Ok(format!("Output file: {}", out_file_path_clone))
    } else {
        Ok(String::from("Convert failed!"))
    }
}

pub fn ncm2mp3(ncm_dir: &str, out_dir: &str) -> Vec<String> {
     let mut results: Vec<String> = vec![];
        ensure_directory_exists(ncm_dir);
        ensure_directory_exists(out_dir);
        wipe_out_ds_store(ncm_dir);
        filter_by_suffix(ncm_dir, "ncm");
        if let Ok(entries) = fs::read_dir(ncm_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let result = process_file(
                        ncm_dir, 
                        out_dir, 
                        entry.file_name().to_str().unwrap()
                    ).unwrap();
                    results.push(result);
                }
            }
        } else {
            eprintln!("Error converting ncm.");
        }

        results
}