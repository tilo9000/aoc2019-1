use std::error::Error;
use std::fs;
use std::process;

fn main() {
    match run("input.txt") {
        Ok(t) => println!("Total Fuel: {}", t),
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1)
        }
    }
}

fn run(filename: &str) -> Result<u64, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let mut m: u64;
    let mut f: u64;
    let mut total: u64 = 0;

    for line in contents.lines() {
        m = line.trim().parse().unwrap();
        f = fuel(m);
        total += f;
        println!("{} --> {}", m, f)
    }
    Ok(total)
}

// Fuel required to launch a given module is based on its mass.
// Specifically, to find the fuel required for a module, take its mass,
// divide by three, round down, and subtract 2.
fn fuel(mass: u64) -> u64 {
    let mut x = mass / 3;
    if x > 2 {
        x -= 2;
        x += fuel(x);   // add the fuel that we need for that mass of fuel
        return x
        };
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_1() {
        assert_eq!(2, fuel(12));
    }
    #[test]
    fn fuel_2() {
        assert_eq!(2, fuel(14));
    }
    #[test]
    fn fuel_3() {
        assert_eq!(966, fuel(1969));
    }
    #[test]
    fn fuel_4() {
        assert_eq!(50346, fuel(100756));
    }
    #[test]
    fn fuel_5() {
        assert_eq!(0, fuel(5));
    }
    #[test]
    fn fuel_z() {
        assert_eq!(0, fuel(0));
    }
}
