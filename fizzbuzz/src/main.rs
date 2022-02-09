use std::{
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    thread::spawn(|| {
        sleep(Duration::from_millis(2_100));
        loop {
            print!("Fizz");
            sleep(Duration::from_secs(3));
        }
    });

    thread::spawn(|| {
        sleep(Duration::from_millis(4_200));
        loop {
            print!("Buzz");
            sleep(Duration::from_secs(5));
        }
    });

    for i in 1.. {
        print!("\n{}\r", &i);
        sleep(Duration::from_secs(1));
    }
}
