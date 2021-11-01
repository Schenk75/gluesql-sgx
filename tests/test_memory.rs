#![cfg(feature = "memory-storage")]
extern crate serde_json;
extern crate serde;
extern crate rand;

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use gluesql::*;
use std::time::{Instant, Duration};
use rand::Rng;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    phone: String
}

fn insert() {
    let storage = MemoryStorage::default();
    let mut glue = Glue::new(storage);
    let _ = glue.execute("CREATE TABLE Test (id INTEGER, name TEXT, phone TEXT)");
    
    let raw_data = std::fs::read_to_string("data.json").unwrap();
    let vals: Vec<Person> = from_str(&raw_data).unwrap();
    for (idx, val) in vals.into_iter().enumerate() {
        // println!("name:{:?}\tphone:{:?}", val.name, val.phone);
        let sql = format!("INSERT INTO Test VALUES ({}, \"{}\", \"{}\")", idx, val.name, val.phone);
        // println!("{}", &sql);
        let _output = glue.execute(&sql).unwrap();
        
        // println!("{:?}", output);
    }
}

#[test]
fn insert_test() {
    let test_num = 10;
    let mut time = Duration::new(0, 0);
    for _ in 0..test_num {
        let start = Instant::now();
        insert();
        time += start.elapsed();
    }
    println!("Insert time: {:?}", time/test_num);
}

#[test]
fn select_test() {
    let test_num = 10;
    let storage = MemoryStorage::default();
    let mut glue = Glue::new(storage);
    let _ = glue.execute("CREATE TABLE Test (id INTEGER, name TEXT, phone TEXT)");
    
    let raw_data = std::fs::read_to_string("data.json").unwrap();
    let vals: Vec<Person> = from_str(&raw_data).unwrap();

    for (idx, val) in vals.into_iter().enumerate() {
        // println!("name:{:?}\tphone:{:?}", val.name, val.phone);
        let sql = format!("INSERT INTO Test VALUES ({}, \"{}\", \"{}\")", idx, val.name, val.phone);
        // println!("{}", &sql);
        let _output = glue.execute(&sql).unwrap();
        // println!("{:?}", output);
    }

    // let mut rng = rand::thread_rng();
    let mut time = Duration::new(0, 0);
    for _ in 0..test_num {
        let start = Instant::now();
        for idx in 0..500 {
            // let idx = rng.gen_range(0..500);
            let sql = format!("SELECT * FROM Test WHERE id = {}", idx);
            let _output = glue.execute(&sql).unwrap();
            // println!("{:?}", output);
        }
        time += start.elapsed();
    }
    println!("Select time: {:?}", time/test_num);
}

#[test]
fn update_test() {
    let test_num = 10;
    let storage = MemoryStorage::default();
    let mut glue = Glue::new(storage);
    let _ = glue.execute("CREATE TABLE Test (id INTEGER, name TEXT, phone TEXT)");
    
    let raw_data = std::fs::read_to_string("data.json").unwrap();
    let vals: Vec<Person> = from_str(&raw_data).unwrap();
    for (idx, val) in vals.into_iter().enumerate() {
        // println!("name:{:?}\tphone:{:?}", val.name, val.phone);
        let sql = format!("INSERT INTO Test VALUES ({}, \"{}\", \"{}\")", idx, val.name, val.phone);
        // println!("{}", &sql);
        let _output = glue.execute(&sql).unwrap();
        // println!("{:?}", output);
    }

    let mut time = Duration::new(0, 0);
    for _ in 0..test_num {
        let start = Instant::now();
        for idx in 0..500 {
            let sql = format!("UPDATE Test SET phone = \"1234\" WHERE id = {}", idx);
            let _output = glue.execute(&sql).unwrap();
        }
        time += start.elapsed();
    }
    
    println!("Update time: {:?}", time/test_num);
}


#[derive(Serialize, Deserialize)]
struct PersonNum {
    name: String,
    numberrange: i32
}

#[test]
fn aggregate_test() {
    let storage = MemoryStorage::default();
    let mut glue = Glue::new(storage);
    let _ = glue.execute("CREATE TABLE Test (id INTEGER, name TEXT, num INTEGER)").unwrap();
    let raw_data = std::fs::read_to_string("data-num10.json").unwrap();
    let vals: Vec<PersonNum> = from_str(&raw_data).unwrap();
    for (idx, val) in vals.into_iter().enumerate() {
        // println!("name:{:?}\tphone:{:?}", val.name, val.phone);
        let sql = format!("INSERT INTO Test VALUES ({}, \"{}\", {})", idx, val.name, val.numberrange);
        // println!("{}", &sql);
        let _output = glue.execute(&sql).unwrap();
        // println!("{:?}", output);
    }

    // let output = glue.execute("SELECT SUM(num) FROM Test").unwrap();
    let output = glue.execute("SELECT * FROM Test ORDER BY num").unwrap();
    println!("{:?}", output)
}