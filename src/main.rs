fn apply_to_jobs(num: u32, title: &str) {
    println!("I'm applying to {num} {title} jobs");
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
     
}

fn main() {
    apply_to_jobs(35, "Rust Developer");

    is_even(8);
    is_even(9);
    println!("Hello, world!");
}
