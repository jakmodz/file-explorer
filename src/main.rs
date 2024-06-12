pub mod ui;
mod searchFile;

use std::path;

use searchFile::fileSystem::{getAll, getCatalogs, getFiles};
use slint::{ModelRc,};
use ui::uic::{*};
slint::include_modules!();

fn main() {
    let main_window = AppWindow::new().unwrap();

   main_window.on_getFiles(|| {
        
        let path: String = String::from("./");
        ModelRc::new(create_model(&getAll(&path)))
    });
   
    main_window.on_onlyFiles(||
    {
        let path: String = String::from("./");
        ModelRc::new(create_model(&getFiles(&path)))  
    });
    main_window.on_onlyCatalogs(||
        {
            let path: String = String::from("./");
            ModelRc::new(create_model(&getCatalogs(&path)))
        });
    main_window.on_both(||
        {
            let path: String = String::from("./");
            ModelRc::new(create_model(&getAll(&path)))
        }
    );
    main_window.run().unwrap();
}


