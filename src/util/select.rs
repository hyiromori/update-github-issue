use dialoguer::{theme::ColorfulTheme, Select};

pub fn select_in_menu<T: ToString + Clone>(description: &String, collection: &Vec<T>) -> Option<T> {
    if collection.is_empty() {
        return None;
    }
    let index: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(description.as_str())
        .default(0)
        .items(&collection)
        .paged(true)
        .interact_opt()
        .unwrap()
        .unwrap();
    Some(collection[index].clone())
}
