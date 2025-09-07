use std::{fs, io, path::Path};

use tracing::{info, warn};

use crate::{
    config::{self, Config},
    parser,
    printer::print_status_overview,
};

const MD_FILE_EXT: &str = "md";

pub struct Context {
    pub config: Config,
}

pub fn get_context() -> Result<Context, Box<dyn std::error::Error>> {
    let config = config::read_config()?;
    return Ok(Context { config: config });
}

pub fn show_status(ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
    let file_paths = get_md_files(Path::new(ctx.config.notes_directory.as_str()))?;
    info!("Found the following md files: {:?}", file_paths);

    let mut file_infos: Vec<parser::FileInfo> = Vec::new();
    for file_path in file_paths {
        let file_info = match parser::parse_file(file_path.clone()) {
            Ok(info) => info,
            Err(e) => {
                warn!("Error occurred in parsing file [{}]: {}", file_path, e);
                continue;
            }
        };
        file_infos.push(file_info);
    }
    print_status_overview(ctx, file_infos);

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
