mod problem1;

pub fn evaluate(name: String) -> Result<f64, &'static str> {
    return match name.as_str() {
        "problem1" => Ok(problem1::main()),
        _ => Err("Problem not found"),
    };
}
