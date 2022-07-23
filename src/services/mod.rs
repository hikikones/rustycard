use std::{
    any::{type_name, Any},
    collections::HashMap,
};

pub mod config;
pub mod database;
pub mod markdown;

pub struct ServiceLocator(HashMap<String, Box<dyn Service>>);

impl ServiceLocator {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert<T: Service>(&mut self, service: T) {
        let type_name = type_name::<T>();
        assert_eq!(self.0.contains_key(type_name), false);
        self.0.insert(type_name.to_owned(), Box::new(service));
    }

    pub fn add<T: Service>(&mut self, service: T) {
        let type_name = type_name::<T>();
        assert_eq!(self.0.contains_key(type_name), false);
        dbg!(type_name);
        self.0.insert(type_name.to_owned(), Box::new(service));
    }

    pub fn get<T: Service>(&self) -> &T {
        let type_name = type_name::<T>();
        assert_eq!(self.0.contains_key(type_name), true);
        dbg!(type_name);
        let s = self.0.get(type_name).unwrap();

        // let s = s as &Box<dyn Any>;

        let a = s.as_ref();
        let b = a.as_any();

        let t = match b.downcast_ref::<T>() {
            Some(a) => a,
            None => panic!("NO SERVICE!!!"),
        };

        t
    }
}

pub trait Service: Any + 'static {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    fn as_any(&self) -> &dyn Any;
}

impl<T> Service for T
where
    T: Any + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// pub trait IntoService {
//     fn into_boxed(self) -> Box<dyn Any>;
// }

// impl<T> IntoService for T
// where
//     T: Any + 'static,
// {
//     fn into_boxed(self) -> Box<dyn Any> {
//         Box::new(self)
//     }
// }

// impl IntoService for Box<dyn IntoService> {
//     fn into_boxed(self) -> Box<dyn Any> {
//         self
//     }
// }
