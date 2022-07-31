use std::{
    any::{type_name, Any},
    collections::HashMap,
};

pub mod config;
pub mod database;
pub mod markdown;

pub struct ServiceLocator(HashMap<String, Box<dyn Any>>);

impl ServiceLocator {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add<T: Any>(&mut self, service: T) {
        let type_name = type_name::<T>();
        self.0.insert(type_name.to_owned(), Box::new(service));
    }

    pub fn get<T: Any>(&self) -> &T {
        let type_name = type_name::<T>();
        let service = self
            .0
            .get(type_name)
            .expect(&format!("Could not locate service {type_name}"));

        service.downcast_ref::<T>().unwrap()
    }
}
