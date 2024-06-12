

pub mod fileSystem
{
    
    use std::string;
    use std::path::{Path,Component};
    
   pub struct fileSystem
    {
      pub Path: String,
      pub CurrentPath: String,
      pub root: char
    }
    pub enum Entry {
        File(String),
        Directory(String),
    }
    impl fileSystem 
    {
      
      
    }
   
    
   fn listAll(path: &String) -> Vec<Entry>
    {
     use std::fs::{read_dir,};
      let mut  vec  = Vec::new();
    
      if let Ok(dir) = read_dir(path) {
         for entry in dir {
             if let Ok(entry) = entry {
                 let metadata = entry.metadata().unwrap();
                 if metadata.is_dir() {
                     if let Some(name) = entry.file_name().to_str() {
                         vec.push(Entry::Directory(name.to_string()));
                     }
                 } else {
                     if let Some(name) = entry.file_name().to_str() {
                         vec.push(Entry::File(name.to_string()));
                     }
                 }
             }
         }
     }
     
      vec
    }
    pub fn getAll(path: &String)-> Vec<String>
    {
     let mut list:Vec<String> = Vec::new();
        let entries =  listAll(path);
        for entry in entries {
         match entry {
             Entry::File(name) => list.push(name),
             Entry::Directory(name) => list.push(name)
         }
     }
      list
    
    }
    pub fn getCatalogs(path: &String) -> Vec<String>
    {
        let mut catalogs:Vec<String> = Vec::new();
        let entries =  listAll(path);
        for entry in entries {
         match entry {
             Entry::File(name) => {}
             Entry::Directory(name) => catalogs.push(name)
         }
     }
       catalogs
    }
    
    pub fn getFiles(path: &String) -> Vec<String>
    {
     let mut files:Vec<String> = Vec::new();
     let entries =  listAll(path);
     for entry in entries {
      match entry {
          Entry::File(name) => files.push(name),
          Entry::Directory(name) =>{}
      }
    }
    files
    }
}