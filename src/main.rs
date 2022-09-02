mod problems;

fn main() {
    let name = std::env::args().nth(1).expect("Must call with a problem number!");
    println!("Calling: {}", name);

    let result = problems::evaluate(name); //.expected("Invalid problem!");
    match result {
        Ok(v) => println!("Got: {}", v),
        Err(e) => println!("Error! {}", e),
    }
}
