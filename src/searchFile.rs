pub mod fileSystem
{
    use crate::create_model;
    use open;
    use slint::VecModel;
    use slint::StandardListViewItem;
    use std::path;
    use std::path::PathBuf;
    use std::path::{Path};
    
   pub struct  fileSystem 
    {
      pub path: String,
    }
    pub enum Entry 
    {
        File(String),
        Directory(String),
    }
    impl fileSystem 
    {
        pub fn new(path:  String) -> Self {
           
            
            fileSystem {path  }
        }
    pub fn getParrentPath(&self) ->Option<String>
    {
        let path = Path::new(&self.path);
    match path.parent() 
    {
        Some(parent) => parent.to_str().map(|s| s.to_string()),
        None => None,
    }
    }
    
    pub fn joinPath(&self,fileName: &String) -> String {
        let base_path = Path::new(&self.path);
        let new_path = base_path.join(fileName);
        new_path.to_string_lossy().to_string()
    }
    pub fn removeLastDir(&mut self)
    {   if self.path  == "/"
        {
            return;
        }
        else
        {
            let path = Path::new(&self.path);
       
            let parent_path = path.parent().unwrap_or(Path::new(""));
            if let Some(parent_str) = parent_path.to_str() {
            self.path = parent_str.to_owned();
            } 
        }
    }
    pub fn checkModel(&self, box1:bool,box2:bool) -> VecModel<StandardListViewItem>
    {
        if box1 
        {
          return create_model(&getFiles(self.path.clone()));
        }
        else if !box1 && !box2 
        {
          return  create_model(&getAll(self.path.clone()));
        }
        else if box1 && box2
        {
            return  create_model(&getAll(self.path.clone()));
        }
        else 
        {
            return  create_model(&getCatalogs(self.path.clone())); 
        }
    }
}
    pub fn openFile(path : &String)
    {
        open::that(path);

    }
    pub fn isFile(path: &String,name : &String) -> bool
    {
        let tring:String =  path.to_string()+"/"+name;
        
        let path_to_check = Path::new(&tring);
        if path_to_check.is_file() 
        {
            return true;    
        }
        else 
        {
            false    
        }
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

pub fn get_extension(file_name: &String) -> String {
    let path = Path::new(file_name);
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext_str| ext_str.to_string())
        .unwrap_or_default()
}
    pub fn getAll(path:String )-> Vec<String>
    {
     let mut list:Vec<String> = Vec::new();
        let entries =  listAll(&path);
        for entry in entries {
         match entry {
             Entry::File(name) => list.push(name),
             Entry::Directory(name) => list.push(name)
         }
     }
      list
    
    }
   pub fn get_last_element_of_path(path_str: &String) -> String {
        let path = Path::new(path_str);
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
            .unwrap_or_default()
    }
    pub fn getCatalogs(path:String) -> Vec<String>
    {
        let mut catalogs:Vec<String> = Vec::new();
        let entries =  listAll(&path);
        for entry in entries {
         match entry {
             Entry::File(name) => {}
             Entry::Directory(name) => catalogs.push(name)
         }
     }
       catalogs
    }
    
    pub fn getFiles(path: String) -> Vec<String>
    {
     let mut files:Vec<String> = Vec::new();
     let entries =  listAll(&path);
     for entry in entries {
      match entry {
          Entry::File(name) => files.push(name),
          Entry::Directory(name) =>{}
      }
    }
    files
}
    pub fn getPathToFile(path:&String, fileName: &String) ->String
    {
            let mut path = PathBuf::from(&path);
            path.push(fileName);
            let path_str = path.to_string_lossy().to_string();
            path_str
    }
    pub fn checkModel(path: &String, box1:bool,box2:bool) -> VecModel<StandardListViewItem>
    {
        if box1 
        {
          return create_model(&getFiles(path.clone()));
        }
        else if !box1 && !box2 
        {
          return  create_model(&getAll(path.clone()));
        }
        else if box1 && box2
        {
            return  create_model(&getAll(path.clone()));
        }
        else 
        {
            return  create_model(&getCatalogs(path.clone())); 
        }
        
}

}