use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    let mut target = String::new();

    print!("Input path: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    print!("Output path: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut output).unwrap();

    print!("Target sled version [0.34]: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut target).unwrap();

    input = input.trim().to_owned();
    output = output.trim().to_owned();
    target = target.trim().to_owned();

    // Default target version
    if target.is_empty() {
        target = "0.34".to_owned();
    }

    let data: Vec<_> = sled_0_31::Config::default()
        .path(input.clone())
        .temporary(true)
        .open()
        .ok()
        .filter(|db| db.was_recovered())
        .map(|db| {
            db.export()
                .into_iter()
                .map(|(a, b, c)| (a, b, Box::new(c) as Box<dyn Iterator<Item = _>>))
                .collect()
        })
        .or_else(|| {
            sled_0_32::Config::default()
                .path(input.clone())
                .open()
                .ok()
                .filter(|db| db.was_recovered())
                .map(|db| {
                    db.export()
                        .into_iter()
                        .map(|(a, b, c)| (a, b, Box::new(c) as Box<dyn Iterator<Item = _>>))
                        .collect()
                })
        })
        .or_else(|| {
            sled_0_33::Config::default()
                .path(input.clone())
                .open()
                .ok()
                .filter(|db| db.was_recovered())
                .map(|db| {
                    db.export()
                        .into_iter()
                        .map(|(a, b, c)| (a, b, Box::new(c) as Box<dyn Iterator<Item = _>>))
                        .collect()
                })
        })
        .or_else(|| {
            sled_0_34::Config::default()
                .path(input.clone())
                .open()
                .ok()
                .filter(|db| db.was_recovered())
                .map(|db| {
                    db.export()
                        .into_iter()
                        .map(|(a, b, c)| (a, b, Box::new(c) as Box<dyn Iterator<Item = _>>))
                        .collect()
                })
        })
        .expect("Your sled db was not found or is not supported");

    match &*target {
        "0.31" => sled_0_31::open(output).expect("Could not create output db").import(data),
        "0.32" => sled_0_32::open(output).expect("Could not create output db").import(data),
        "0.33" => sled_0_33::open(output).expect("Could not create output db").import(data),
        "0.34" => sled_0_34::open(output).expect("Could not create output db").import(data),
        _ => panic!("Unsupported target version")
    }
}
