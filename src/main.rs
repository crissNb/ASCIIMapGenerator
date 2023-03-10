use std::io::prelude::*;
use std::{fs::OpenOptions, io::BufWriter};

use rand::Rng;

pub mod map_generator;

fn main() {
    let mut rng = rand::thread_rng();

    println!("--------------------------");
    println!(
        "Desired map width and height (leave empty (or invalid input) for random between 0 - 100):"
    );

    println!("Height:");
    let map_width: usize = match get_input().trim().parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid input provided, defaulting to random value between 1..100");
            rng.gen_range(1..100)
        }
    };

    println!("Width:");
    let map_height: usize = match get_input().trim().parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid input provided, defaulting to random value between 1..100");
            rng.gen_range(1..100)
        }
    };

    println!("--------------------------");
    println!("Map count:");
    let map_count: usize = match get_input().trim().parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid input provided, defaulting to 1");
            1
        }
    };

    println!("--------------------------");
    println!("Output location (leave empty for printing out in console):");
    let output_loc = get_input();

    println!("--------------------------");
    println!("Generating...");

    if output_loc.trim().is_empty() {
        for _i in 0..map_count {
            let result: Vec<Vec<char>> = map_generator::render_map(
                &map_generator::generate(map_width, map_height, 0.8, 98),
                map_width,
                map_height,
            );

            // Print to console
            for row in &result {
                for element in row {
                    print!("{}", element);
                }
                println!();
            }
            println!("================================");
        }
    } else {
        // Save to a file
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(output_loc.trim())
            .unwrap();
        let mut buffile = BufWriter::new(file);

        for _i in 0..map_count {
            let result: Vec<Vec<char>> = map_generator::render_map(
                &map_generator::generate(map_width, map_height, 0.8, 98),
                map_width,
                map_height,
            );

            // Print to file
            for row in &result {
                for element in row {
                    if let Err(e) = write!(buffile, "{}", element) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
                if let Err(e) = writeln!(buffile) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }

            if let Err(e) = writeln!(buffile, "================================") {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
}

// fn test_subprocess() -> String {
//     let mut process = Command::new("java")
//         .arg("TestJavaClass")
//         .stdin(Stdio::piped())
//         .spawn()
//         .expect("Failed to start Java subprocess");
//
//     let stdin = process.stdin.as_mut().expect("Failed to get stdin handle");
//
//     stdin
//         .write_all(b"test\ntest\n")
//         .expect("Failed to write to stdin");
//
//     stdin.flush().expect("Failed to flush stdin");
//
//     let stdout = process
//         .stdout
//         .as_mut()
//         .expect("Failed to get stdout handle");
//
//     let mut buffer = Vec::new();
//     stdout
//         .read_to_end(&mut buffer)
//         .expect("Failed to read stdout");
//
//     let output = String::from_utf8(buffer).expect("Failed to convert output to string");
//     return output;
// }

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}
