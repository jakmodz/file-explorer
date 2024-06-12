
    pub mod uic
    {
        use slint::{SharedString, StandardListViewItem, VecModel};
       

        pub fn create_list_item(text: &str) -> StandardListViewItem {
            let mut item = StandardListViewItem::default();
            // Set the text property of the StandardListViewItem
            item.text = SharedString::from(text);
            item
        }
        
        pub fn create_model(vec: &Vec<String>) -> VecModel<StandardListViewItem> {
            let mut model = VecModel::default();
            
            // Populate the model
            for text in vec.iter() {
                model.push(create_list_item(text));
            }
            
            model
        }
        
    }
