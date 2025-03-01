use std::sync::{Arc, Mutex};
use superdsp::{Frequency, Stage};

fn test() {
    println!("hi!")
}

fn cookie(frequency: Frequency){
    println!("cookie frequency is {}", frequency);
}

fn main() {
    test.invoke(());
    
    cookie.invoke(300 as Frequency);
}