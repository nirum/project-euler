use std::time::Instant;
mod problems;

fn main() {
    let name = std::env::args().nth(1).expect("Must call with a problem number!");
    println!("Calling: {}", name);

    let now = Instant::now();
    let result = problems::evaluate(name);
    let runtime = now.elapsed().as_millis();

    match result {
        Ok(v) => println!("Got: {}. Took {} ms.", v, runtime),
        Err(e) => println!("Error! {}", e),
    }
}
