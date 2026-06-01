use std::{collections::HashMap, sync::{Arc, Mutex}, thread};

pub enum TrafficLight { 
    Red, Green, Yellow
 }

pub fn next_state(light: &TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Red => TrafficLight::Green,
        TrafficLight::Green => TrafficLight::Yellow,
        TrafficLight::Yellow => TrafficLight::Red,
    }
}

pub fn name(light: &TrafficLight) -> &str {
    match light {
        TrafficLight::Red => "Red",
        TrafficLight::Green => "Green",
        TrafficLight::Yellow => "Yellow",
    }
}

pub fn simulate(steps: usize) -> Vec<String> {
    let mut light = TrafficLight::Red;
    let mut result = vec![name(&light).to_string()];
    for _ in 0..steps {
        light = next_state(&light);
        result.push(name(&light).to_string());
    }
    result
}

pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

pub fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Neg(e) => -eval(e),
    }
}


pub fn cached_squares(inputs: Vec<i32>) -> Vec<i32> {
    let cache = Arc::new(Mutex::new(HashMap::new()));

    let mut unique: Vec<i32> = inputs.clone();
    unique.sort();
    unique.dedup();

    let handles: Vec<_> = unique.into_iter().map(|x| {
        let c = Arc::clone(&cache);
        thread::spawn(move || { c.lock().unwrap().insert(x, x * x); })
    }).collect();

    for h in handles { h.join().unwrap(); }

    let cache = cache.lock().unwrap();
    inputs.iter().map(|x| *cache.get(x).unwrap()).collect()
}