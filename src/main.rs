pub mod ui;
mod searchFile;

use std::sync::{Arc,Mutex};
use slint::StandardListViewItem;
use searchFile::fileSystem::{fileSystem, getAll, getCatalogs, getFiles, getPathToFile, openFile};
use slint::{ModelRc};
use ui::uic::{*};
slint::include_modules!();

fn main() {
    
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
      main_window.on_both(move || {
          let fs = file_system.lock().unwrap();
          ModelRc::new(create_model(&getAll(fs.path.clone())))
      });
  }
  {
      let file_system = Arc::clone(&file_system);
      main_window.on_moveForward(move |item: StandardListViewItem| {
          let mut fs = file_system.lock().unwrap();
          let file_name = item.text.to_string();  
          if !fs.isFile(&file_name) {
             fs.path =  fs.joinPath(&file_name);
            
          }

          ModelRc::new(create_model(&getAll(fs.path.clone())))
      });

  }
  {
    
    let file_system = Arc::clone(&file_system);
    main_window.on_openFile(move |item:StandardListViewItem |
    {
        let mut fs = file_system.lock().unwrap();
        let file_name = item.text.to_string();
            
            open::that(getPathToFile(&fs.path, &file_name));
  
    });
  }
  {
    let file_system = Arc::clone(&file_system);
    main_window.on_moveBack(move ||
    {
        let mut fs = file_system.lock().unwrap();
        fs.removeLastDir();
        
        ModelRc::new(create_model(&getAll(fs.path.clone())))
        
    }
    );
  }
  

    main_window.run().unwrap();
}


