use std::io;

fn main() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).expect("Failed to read line");

    let num_a: i32 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error");
            return;
        }
    };

    let mut b = String::new();

    io::stdin().read_line(&mut b).expect("Failed to read line");

    let num_b: i32 = match b.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error");
            return;
        }
    };

    println!("X = {}", num_a + num_b);
}
