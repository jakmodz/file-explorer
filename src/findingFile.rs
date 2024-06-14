
pub mod findingFile
{
    use rayon::prelude::*;
    use std::fmt::Error;
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
    pub  fn searchDirForFile(path:&String,fileName: &String) -> FileOption
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

}