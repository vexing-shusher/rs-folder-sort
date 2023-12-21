use std::{io, fs, path::{PathBuf, Path}};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    // read user input: path to dir that needs to be sorted
    let mut folder: String = String::new();

    // initialize naming map
    let name_map = get_name_map();

    let example_path: String = "/path/to/my_folder".to_string();

    println!("Enter the full path to the folder.\n");
    println!("The path should be in the following format: {}", format_path(&example_path).display());

    loop {
        // if we don't clear it explicitly and enter the wrong value first,
        // the loop will get stuck regardless of subsequent inputs
        folder.clear();

        match io::stdin().read_line(&mut folder) {
            Ok(_result) => {
                if check_path(&folder) {
                    break;
            }
            else {
                println!("Please, enter a valid path.");
            }
        },
            Err(_) => println!("Please, enter a valid path.")
        }
    };
    

    match listdir(&format_path(&folder)) {
        Ok(contents) => for path in contents {
            if path.is_dir() {
                continue
            }
            else {
                // get file extension
                let ext_ = path.extension()
                .expect("Could not determine file extension.")
                .to_str()
                .unwrap()
                .to_uppercase();

                let ext: String = name_map.get(&ext_).unwrap_or(&ext_).to_string();

                if ext == "INI" {
                    println!("Can't move system files with .ini extension.");
                    continue
                }
                else {
                    let parent = path.parent().unwrap();

                    let fname: &str = path.file_name().unwrap().to_str().unwrap();
                    
                    // create extension folder
                    let directory_name = format!("{}/{}/", parent.display(), ext);
                    let directory_buf: PathBuf = format_path(&directory_name);
                    
                    if !directory_buf.is_dir()
                    {
                        match fs::create_dir(&directory_name) {
                            Ok(_) => println!("Directory '{}' created successfully.", directory_buf.display()),
                            Err(err) => eprintln!("Error creating directory: {}", err),
                        }
                    }
                    else {
                        println!("Directory {} exists.", directory_buf.display())
                    };

                    // move the file into the new directory
                    let out_path = format!("{}/{}", directory_name, fname);
                    let out_buf: PathBuf = convert_path_to_os_style(&out_path);

                    fs::rename(path.clone(), out_buf.clone())?;

                    println!("File moved to {}", out_buf.display());
                }
            }
        },
        Err(err) => panic!("\n{}\n", err)
    }

    Ok(())

}

fn listdir(folder: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = fs::read_dir(folder)?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    Ok(entries)
}

fn check_path(path: &String) -> bool {
    if path.trim().len() == 0 {
        println!("Path cannot be empty.");
        return false;
    }

    let formatted_path = format_path(path);
    if !formatted_path.is_dir() {
        println!("{} is not a directory", formatted_path.display());
        return false;
    }
    true
}

fn format_path(path: &String) -> PathBuf {
    // trim the path
    let mut path_: String = path.trim().to_string();
    // check that it ends in a trailing slash
    let path_len: usize = path_.len();
    if &path_[path_len-2..path_len-1] != r"\" || &path_[path_len-2..path_len-1] != "/" {
        path_.push_str(r"\")
    };
    let path_buf: PathBuf = convert_path_to_os_style(&path_);
    path_buf
}

// Function to convert the path to the appropriate format for the current OS
fn convert_path_to_os_style(path: &String) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let path_: String = path.replace("/", r"\");
        // On Windows, convert path separators to backslashes
        Path::new(&path_).to_path_buf()
    }

    #[cfg(not(target_os = "windows"))]
    {
        let path_: String = path.replace(r"\", "/");
        // On Unix-like systems, keep path separators as forward slashes
        Path::new(path).to_path_buf()
    }
}

fn get_name_map() -> HashMap<String, String> {
    let mut name_map: HashMap<String, String> = HashMap::new();
    let text_keys: Vec<&str> = vec!["TXT", "DOC", "DOCX", "ODT"];
    let tabular_keys: Vec<&str> = vec!["CSV", "ODT", "XLS", "XLSX"];
    let ppt_keys: Vec<&str> = vec!["PPT", "PPTX", "ODP"];
    let image_keys: Vec<&str> = vec!["JPG", "JPEG", "PNG", "TIFF", "BMP"];
    let video_keys: Vec<&str> = vec!["MP4", "MOV", "AVI", "WMV"];
    let audio_keys: Vec<&str> = vec!["WAV", "MP3", "FLAC"];

    for key in text_keys {
        name_map.insert(key.to_string(), "Texts".to_string());
    }
    for key in tabular_keys {
        name_map.insert(key.to_string(), "Tables".to_string());
    }
    for key in ppt_keys {
        name_map.insert(key.to_string(), "Presentations".to_string());
    }
    for key in image_keys {
        name_map.insert(key.to_string(), "Images".to_string());
    }
    for key in video_keys {
        name_map.insert(key.to_string(), "Videos".to_string());
    }
    for key in audio_keys {
        name_map.insert(key.to_string(), "Audios".to_string());
    }
    name_map
}

