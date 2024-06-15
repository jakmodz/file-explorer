
pub mod findingFile
{
    use rayon::prelude::*;
    use slint::SharedString;
    use std::fs;
    use slint::{StandardListViewItem,VecModel};
    use std::path::{Path, PathBuf};
    use std::sync::mpsc;
    use slint::ModelRc;
    use crate::{create_model, create_model_with_single_item};
   use crate::searchFile::fileSystem::{*};
   pub enum FileOption 
    {
       PATH(String),
       ERR(String)

    }
    pub fn searchDirForFile(path:&String,fileName: &String) -> String
    {
        let vec = getFiles(path.to_string());
        for file in vec.iter()
        {
            if file == fileName
            {
                let finalPath: String = path.to_owned()+"/" +fileName;
                return finalPath;
            }
        } 
        return  "".to_string();
    }
    pub fn find_specific_file_in_dir(dir: &Path, target_file_name: &str) -> Option<String> {
        if let Ok(entries) = fs::read_dir(dir) 
        {
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
    pub  fn doAll(path:&mut String,fileName:SharedString) ->VecModel<StandardListViewItem>
    {
         
       let v  =  searchDirForFile(path,  &fileName.to_string()) ;
       if v.len()> 0  
       {
           return create_model_with_single_item(&fileName.to_string());
       }
       else 
       {
            
            let catalogs =  getCatalogs(path.clone());
            for catalog in catalogs
            {
                let mut  string:String =String::new();
                let mut  pathtem = path.clone() + "/"+&catalog;
               let pathh =  PathBuf::from(path.clone());
               match  find_specific_file_in_dir(&pathh, &fileName) 
               {
                   Some(p) => string= p,
                   None=> string = String::from("")
               };
               if string.len() > 0
               {
                    *path = string;
                    return (create_model_with_single_item(&fileName.to_string()));
               }
               else  
               {
                   continue;
               }
            } 
            
           create_model(&getAll(path.to_string()))
       }
       
    } 
    }

 