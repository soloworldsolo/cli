use std::any::type_name;
use std::fmt::format;

pub trait Animal {
    fn makes_sound(&self) -> String;
    fn new ( name:String, color:String) -> Self;
}

pub struct Dog {
    name: String,
    color: String,
}


  impl Animal for Dog {
    fn makes_sound(&self) -> String {
        let mut name = String::from(&self.name);
         name.push_str(&self.color);
        return  name;
    }

     fn new(name:String , color:String) -> Self {
        Dog {
            name,
            color
        }
    }
}