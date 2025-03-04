use std::{fs::File, io::Write};
use ncmdump::Ncmdump;
use std::{fs, path::Path};
use neon::{
    prelude::{Context, FunctionContext, JsString}, 
    result::JsResult, types::JsArray
};

use crate::utils::vec_to_array;

pub fn transform(ncm_dir: &str, out_dir: &str, file_name: &str) -> std::io::Result<String> {
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

fn ensure_directory_exists(dir: &str) -> () {
    let path = Path::new(dir);
    if !path.exists() {
        match fs::create_dir(&path) {
            Ok(_) => println!("Created directory: {}", dir),
            Err(e) => println!("Error creating directory: {}", e),
        }
    }
}

fn filter_by_suffix(dir: &str, suffix: &str) {
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

fn wipe_out_ds_store(dir: &str) -> () {
    let path = format!("{}/{}", dir, ".DS_Store");
    let file_exists = fs::exists(&path).expect("File doesn't exists!");
    if file_exists {
        fs::remove_file(path).expect("Can't remove file as file doesn't exists");
    }
}

// export js func
pub fn process_file(mut cx: FunctionContext) -> JsResult<JsString> {
    let ncm_dir = cx.argument::<JsString>(0)?.value(&mut cx);
    let out_dir = cx.argument::<JsString>(1)?.value(&mut cx);
    let file_name = cx.argument::<JsString>(2)?.value(&mut cx);

    let handle_result = transform(&ncm_dir,&out_dir, &file_name);

    match handle_result {
        Ok(res) => {
            return Ok(cx.try_string(res).unwrap());
        },
        Err(err) => {
            return Ok(cx.try_string(err.to_string()).unwrap());
        }
    }
}

// export js func
pub fn ncm2mp3(mut cx: FunctionContext) -> JsResult<JsArray> {
    let ncm_dir = cx.argument::<JsString>(0)?.value(&mut cx);
    let out_dir = cx.argument::<JsString>(1)?.value(&mut cx);

    let ncm_dir_clone = ncm_dir.clone();
    let out_dir_clone = out_dir.clone();

    let mut results: Vec<String> = vec![];
    ensure_directory_exists(&ncm_dir);
    ensure_directory_exists(&out_dir);

    wipe_out_ds_store(&ncm_dir);
    filter_by_suffix(&ncm_dir, "ncm");
    if let Ok(entries) = fs::read_dir(ncm_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let result = transform(
                    &ncm_dir_clone, 
                    &out_dir_clone, 
                    entry.file_name().to_str().unwrap()
                ).unwrap();
                results.push(result);
            }
        }
    } else {
        eprintln!("Error converting ncm.");
    }

    vec_to_array(&results, &mut cx)
}