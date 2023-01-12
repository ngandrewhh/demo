use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use rand::prelude::*;
use std::io;

/*
    Rust Book Section 8.3 Exercises
 */

fn vec_exercise() {
    println!("Section 8.3 Exercise 1");

    let mut rng = rand::thread_rng();
    let mut v = Vec::new();

    // create random numbers
    for _ in 0..1000 {
        v.push(rng.gen_range(0..50));
    }

    // find median
    v.sort();
    println!("median is {}", (v[499] + v[500]) / 2);

    // find mode
    let mut h: HashMap<i32, i32> = HashMap::new();
    let (mut max_k, mut max_v)= (-1, 0);
    
    for &i in v.iter() {
        let count = h.entry(i).or_insert(0);
        *count += 1;

        if *count > max_v {
            max_k = i;
            max_v = *count;
        }
    }

    println!("mode is {}\n", &max_k);

    // print out all items 
    for i in v[0]..=v[v.len() - 1] {
        match i {
            _ if i == max_k => println!("{:?}: [[{:?}]] ** mode", i, h.get(&i).copied().unwrap_or(0)),
            _ => println!("{:?}: {:?}", i, h.get(&i).copied().unwrap_or(0)),
        }
    }
}

fn add_employee(emp: &mut HashMap<String, String>, dpt: &mut HashMap<String, Vec<String>>) {
    let mut emp_name = String::new();
    let mut emp_dept = String::new();

    println!("\nPlease enter employee name:");
    io::stdin().read_line(&mut emp_name);
    emp_name = emp_name.trim().to_string();

    println!("\nPlease enter employee department:");
    io::stdin().read_line(&mut emp_dept);
    emp_dept = emp_dept.trim().to_string();

    let success = match emp.entry(emp_name.clone()) {
        Occupied(_) => { 
            println!("Adding failed!");
            false
        },
        Vacant(e) => { 
            println!("\nAdding [{}] to [{}] department", emp_name, emp_dept);
            e.insert(emp_dept.clone()); 
            true
        },
    };

    if !success { return; }

    match dpt.entry(emp_dept.clone()) {
        Occupied(mut e) => { 
            e.get_mut().push(emp_name); 
        },
        Vacant(e) => { 
            e.insert(vec![emp_name]); 
        },
    }
}

fn print_emp(emp: &HashMap<String, String>) {
    let mut k = Vec::from_iter(emp.keys());
    k.sort();

    for &emp_name in k.iter() {
        println!("Employee [{}] is in [{}] department", emp_name, emp[emp_name])
    } 
}

fn print_dept(dpt: &HashMap<String, Vec<String>>) {
    let mut k = Vec::from_iter(dpt.keys());
    k.sort();

    for &dpt_name in k.iter() {
        println!("Employee {:?} is in [{}] department", dpt[dpt_name], dpt_name)
    } 
}

fn del_employee(emp: &mut HashMap<String, String>, dpt: &mut HashMap<String, Vec<String>>) {
    let mut emp_name = String::new();

    println!("\nPlease enter employee name:");
    io::stdin().read_line(&mut emp_name);
    emp_name = emp_name.trim().to_string();

    match emp.remove(&emp_name) {
        Some(T) => {
            let index = dpt[&T].iter().position(|x| *x == emp_name).unwrap();
            dpt.get_mut(&T).unwrap().remove(index);
        },
        None => (),
    }
}

fn cli_exercise() {
    println!("Section 8.3 Exercise 3");

    // Init
    let mut emp: HashMap<String, String> = HashMap::new();
    let mut dpt: HashMap<String, Vec<String>> = HashMap::new();

    // Test Data
    emp.insert(String::from("Andrew"), String::from("Sales"));
    emp.insert(String::from("Becky"), String::from("Engineering"));
    emp.insert(String::from("Zeno"), String::from("Sales"));

    dpt.insert(String::from("Sales"), vec![String::from("Andrew"), String::from("Zeno")]);
    dpt.insert(String::from("Engineering"), vec![String::from("Becky")]);

    // Driver
    println!("Welcome to employee directory.");
    println!("Press any key to continue...");

    let mut inp = String::new();
    io::stdin().read_line(&mut inp);

    // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    loop {
        println!("\nOptions");
        println!("------------------------------");
        println!("1. Add employee data");
        println!("2. Delete employee data");
        println!("3. Display data by employee");
        println!("4. Display data by department");
        println!("5. Exit");

        let mut inp = String::new();
        io::stdin().read_line(&mut inp);

        let opt: i32 = inp.trim().parse().unwrap_or(0);

        match opt {
            1 => add_employee(&mut emp, &mut dpt),
            2 => del_employee(&mut emp, &mut dpt),
            3 => print_emp(&emp),
            4 => print_dept(&dpt),
            5 => break,
            _ => println!("Invalid input! Please try again"),
        }
    }

}

fn main() {
    vec_exercise();
    cli_exercise();
}