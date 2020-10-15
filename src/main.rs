use std::{env, io::{stdin, stdout, Write}};
use std::collections::BTreeMap;

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    let mut target = String::new();

    let args: BTreeMap<String, String> = env::args()
        .filter(|arg| arg.starts_with("-"))
        .map(|arg| {
            arg.splitn(2, '=')
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .fold(BTreeMap::new(), |mut map, arg| {
            if arg.len() != 2 {
                panic!("Wrong argument! An argument must be of the format 'key=value'");
            }
            map.insert(arg[0].trim_start_matches('-').to_owned(), arg[1].to_owned());
            map
        });

    if let Some(input_path) = args.get("input") {
        input = input_path.to_owned();
    } else {
        print!("Input path: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
    }

    if let Some(output_path) = args.get("output") {
        output = output_path.to_owned();
    } else {
        print!("Output path: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut output).unwrap();
    }

    if let Some(target_version) = args.get("target") {
        target = target_version.to_owned();
    } else {
        print!("Target sled version [0.34]: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut target).unwrap();
    }

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
