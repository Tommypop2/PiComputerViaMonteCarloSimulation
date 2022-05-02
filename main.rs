use rand::prelude::*;
use std::io;
use std::process;
use std::thread;
fn pause() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");
}

fn thread_creator(nb_threads: usize, iterations: i64) -> Vec<std::thread::JoinHandle<[i64; 2]>> {
    let mut v = Vec::new();
    for _n in 0..nb_threads {
        v.push(thread::spawn(move || {
            let res = compute(iterations);
            println!("Thread {:?} has finished.", thread::current().id());
            res
        }));
    }
    v
}
fn main() {
    println!("Enter the number of threads that the program should run on:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nb_threads = input.trim().parse::<i32>().unwrap_or_else(|_| {
        println!("Input is not an integer");
        drop(input);
        process::exit(1);
    });
    println!("Enter the number of iterations per thread that the program should run:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let iterations = input.trim().parse::<i64>().unwrap_or_else(|_| {
        println!("Input is not an integer");
        drop(input);
        process::exit(1);
    });
    println!("{} threads have been created", nb_threads);
    let mut vec: Vec<[i64; 2]> = Vec::new();
    for th in thread_creator(nb_threads as usize, iterations) {
        vec.push(th.join().unwrap());
    }
    let mut inner: i64 = 0;
    let mut total: i64 = 0;
    for i in vec {
        inner += i[0];
        total += i[1];
    }
    let res: f64 = 4.0 * (inner as f64 / total as f64);
    println!("{}", res);
    pause();
}
fn square(var: f64) -> f64 {
    var * var
}
fn compute(iterations: i64) -> [i64; 2] {
    let mut counter: i64 = iterations;
    let mut rng = rand::thread_rng();
    let mut distance: f64;
    let mut inner: i64 = 0;
    while counter > 0 {
        distance = ((square(rng.gen()) + square(rng.gen())) as f64).sqrt();
        if distance < 1.0 {
            inner += 1;
        }
        counter -= 1;
    }
    [inner, iterations]
}
