pub mod filesAddAndDelete
{
    use std::{fs, module_path, path};
    use std::fs::{remove_dir, remove_dir_all, remove_file, File};
    pub fn deleteFile(path:&String)
    {
        let res = remove_file(path);
        match res
       {
        Ok(File) => {},
        Err(e) => {}       }
    }
    pub fn deltetDir(path:&String)
    {
        
        remove_dir_all(path);
       
    }
    pub fn addFile(path:&String,name: &String)
    {
        let finalPath = path.to_owned()+"/"+name;
       let res =  File::create(finalPath);
       match res
       {
           Ok(File) => {},
           Err(e) => {}
       }
    }
    pub fn addDir(path:&String,name: &String)
    {
        let finalPath = path.to_owned()+"/"+name;
       let res =  fs::create_dir(finalPath);
       match res
       {
        Ok(File) => {},
        Err(e) => {}
       }
    }
    pub fn addFinal(path: &String,fileName: &String,fileOrDir:bool)
    {
        if fileOrDir 
        {
            addFile(path, fileName);     
        }
        else
        {
            addDir(path, fileName);
        }
    }
    
}