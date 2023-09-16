mod hof;
mod traitex;

extern crate core;

use crate::Color::{RED,GREEN,BLUE};
use crate::traitex::{Animal, Dog};


fn main() {
let mut vanavil = ["violet" ,"indigo", "blue","green"];
 let person = ("solomon", 28, 75_000000.89);
  println!("{:?}",person);
  let (name ,age,salary ) = person;
  println!("{} {} {}",name ,age,salary);

 let product = Product  {
    name :String::from("Prodvec") ,
    cost: 2.0,
    country: String::from("India")
  };
  println!("{:?}", product);
 match_color(Color::RED);
  match_color(Color::GREEN);
  match_color(Color::BLUE);
    let lamb = |x |-> i32 {x*x};
    let res = lamb(7);
    println!("{}", res);
    hello_world!();
hof::hof();
    let doberman: Dog = Dog::new(String::from("doberman") ,String::from("black"));
    println!("{:?}",doberman.makes_sound())
}

#[derive(Debug)]
struct Product {
  name :String,
  cost: f32,
  country: String
}

#[derive(Debug)]
enum Color {
  RED ,
  GREEN,
  BLUE


}
fn match_color(color_code:Color) {
  match   color_code {
    RED => {println!("{}",String::from("wassup"))} ,
    GREEN => {println!("{}",String::from( "geen cgkkjk"))},
    BLUE => {println!("{}",String::from("blueee"))}
  }
}
#[macro_export]
 macro_rules! hello_world {
    () => {println!("{}",String::from( "imagine a virus"))}
}
