
pub mod findingFile
{
    use rayon::prelude::*;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::mpsc;
    use std::thread;
    use std::path;
   
   use crate::searchFile::fileSystem::{*};
    enum FileOption 
    {
       PATH(String),
       ERR(String)

    }
    pub async  fn searchDirForFile(path:&String,fileName: &String) -> FileOption
    {
        let vec = getFiles(path.to_string());
        for file in vec.iter()
        {
            if file == fileName
            {
                let finalPath: String = path.to_owned()+"/" +fileName;
                return FileOption::PATH((finalPath));
            }
        } 
        FileOption::ERR(String::from("couldnt find file"))
    }
    pub fn find_specific_file_in_dir(dir: &Path, target_file_name: &str) -> Option<String> {
        if let Ok(entries) = fs::read_dir(dir) {
            let paths: Vec<PathBuf> = entries
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .collect();
    
            // Check files in the current directory
            if let Some(found_file) = paths.iter().find(|path| {
                path.is_file() && path.file_name().map_or(false, |name| name == target_file_name)
            }) {
                return Some(found_file.to_string_lossy().to_string());
            }
    
            // Check subdirectories in parallel
            let dirs: Vec<_> = paths.into_iter().filter(|path| path.is_dir()).collect();
            let subdir_results: Vec<_> = dirs
                .par_iter()
                .filter_map(|subdir| find_specific_file_in_dir(subdir, target_file_name))
                .collect();
    
            if !subdir_results.is_empty() {
                return Some(subdir_results[0].clone());
            }
        }
        None
    }
    }

 