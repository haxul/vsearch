use std::fmt::Display;

pub trait Storage {
    fn save(&self, data_list: Vec<impl Display>) -> bool;

    fn get_storage_name(&self) -> &str;
}