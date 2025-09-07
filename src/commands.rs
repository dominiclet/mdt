use std::{fs, io, path::Path};

use crate::{config::Config, parser, printer::print_status_overview};

const MD_FILE_EXT: &str = "md";

pub fn show_status(conf: Config) -> Result<(), Box<dyn std::error::Error>> {
    let file_paths = get_md_files(Path::new(conf.notes_directory.as_str()))?;

    let mut file_infos: Vec<parser::FileInfo> = Vec::new();
    for file_path in file_paths {
        let file_info = match parser::parse_file(file_path) {
            Ok(info) => info,
            Err(_e) => {
                // TODO: log error
                continue;
            }
        };
        file_infos.push(file_info);
    }
    print_status_overview(file_infos, conf);

    Ok(())
}

fn get_md_files(dir: &Path) -> io::Result<Vec<String>> {
    if dir.is_dir() {
        let mut files: Vec<String> = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(get_md_files(&path)?);
                continue;
            }
            let file_ext = match path.extension() {
                Some(name) => name,
                None => continue,
            };
            if file_ext == MD_FILE_EXT {
                if let Some(file_path) = path.to_str() {
                    files.push(file_path.to_string());
                };
            }
        }
        return Ok(files);
    }
    Ok(Vec::new())
}
