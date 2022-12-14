mod problem1;
mod problem2;
mod problem19;
mod problem36;
mod problem54;

pub fn evaluate(name: String) -> Result<f64, &'static str> {
    return match name.as_str() {
        "problem1" => Ok(problem1::main()),
        "problem2" => Ok(problem2::main()),
        "problem19" => Ok(problem19::main()),
        "problem36" => Ok(problem36::main()),
        "problem54" => Ok(problem54::main()),
        _ => Err("Problem not found"),
    };
}
