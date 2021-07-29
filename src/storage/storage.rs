use std::fmt::Display;

pub trait Storage {
    fn save(&self, row: impl Display);

    fn get_storage_name(&self) -> &str;
}