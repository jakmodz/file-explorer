pub mod ui;
mod filesAddAndDelete;
use std::sync::{Arc,Mutex};
use filesAddAndDelete::filesAddAndDelete::{deleteFile, deltetDir};
use slint::StandardListViewItem;
use searchFile::fileSystem::{fileSystem, getAll, getCatalogs, getFiles, getPathToFile, isFile};
use slint::{ModelRc};
use ui::uic::{*};
slint::include_modules!();
mod searchFile;
fn main() 
{    
   let main_window = AppWindow::new().unwrap();
   let file_system = Arc::new(Mutex::new(fileSystem::new(String::from("/"))));
  {
      let file_system = Arc::clone(&file_system);
      main_window.on_getFiles(move || {
          let fs = file_system.lock().unwrap();
          ModelRc::new(create_model(&getAll(fs.path.clone())))
      });
  }
  {
      let file_system = Arc::clone(&file_system);
      main_window.on_onlyFiles(move || {
          let fs = file_system.lock().unwrap();
          ModelRc::new(create_model(&getFiles(fs.path.clone())))
      });
  }
  {
      let file_system = Arc::clone(&file_system);
      main_window.on_onlyCatalogs(move || {
          let fs = file_system.lock().unwrap();
          ModelRc::new(create_model(&getCatalogs(fs.path.clone())))
      });
  }

  
  {
      let file_system = Arc::clone(&file_system);
      main_window.on_moveForward(move |item: StandardListViewItem, box1:bool, box2:bool| 
        {
          let mut fs = file_system.lock().unwrap();
          let file_name = item.text.to_string();  
          use crate::searchFile::fileSystem::isFile;
          if !isFile(&fs.path,&file_name) 
          {
             fs.path =  fs.joinPath(&file_name);
            
          }
          
          ModelRc::new(fs.checkModel(box1, box2))  
      });

  }
  {
    
    let file_system = Arc::clone(&file_system);
    main_window.on_openFile(move |item:StandardListViewItem |
    {
        let fs = file_system.lock().unwrap();
        let file_name = item.text.to_string();
            
            open::that(getPathToFile(&fs.path, &file_name));
  
    });
  }
  {
    let file_system = Arc::clone(&file_system);
    main_window.on_moveBack(move |box1:bool, box2:bool|
    {
        let mut fs = file_system.lock().unwrap();
        fs.removeLastDir();

        ModelRc::new(fs.checkModel(box1, box2))   
    }
    );
  }
  {
    let file_system = Arc::clone(&file_system);
    main_window.on_deleteFile(move |item: StandardListViewItem|
    {
        let mut fs = file_system.lock().unwrap();
        let file_name = item.text.to_string(); 
        let path =getPathToFile(&fs.path, &file_name);

        if !isFile(&fs.path, &file_name)
        {
            deltetDir(&path);
        }
        else if  isFile(&fs.path, &file_name)
        {
            deleteFile(&path);    
        }          
    }
    );
  }
    main_window.run().unwrap();
}


