use std::f64::consts::PI;

pub struct Circle {
    radius : f64
}

pub trait Shape {
    fn area(&self) -> f64;
}
pub struct Rectangle {
    w : f64,
    h : f64
}

impl Shape for Circle {
    
    fn area(&self) -> f64 {
        return PI * (&self.radius * &self.radius);
    }

}


impl Shape for Rectangle {
    
    fn area(&self) -> f64 {
        return &self.w * &self.h;
    }

}

pub fn total_area(shapes : &[Box<dyn Shape>]) -> f64{

    return shapes.iter().map(|x|x.area()).sum();

}

/////////////////////// 
pub trait Formatter {
    fn format(&self , input : &str)-> String;

}

pub struct Upper;
pub struct Snake;
pub struct Trim;

impl  Formatter for Upper {

    fn format(&self , input : &str)-> String {
        return input.to_uppercase();
    }


}   


impl  Formatter for Snake {

    fn format(&self , input : &str)-> String {
        return input.replace(' ' , "_");
    }


}   

impl  Formatter for Trim {

    fn format(&self , input : &str)-> String {
        return input.trim().to_string();
    }


}   


fn apply_all(input: &str, fmts: &[Box<dyn Formatter>]) -> String{

    fmts.iter().fold(input.to_string(), |acc,f|f.format(&acc))

}