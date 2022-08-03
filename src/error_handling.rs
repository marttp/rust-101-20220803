use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

pub fn example() {
    // When the panic! memory is cleaned up and the program quits
    // panic!("Terrible Error");
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file : {:?}", error);
        }
    };

    // Write to file and define the panic! error message with expect
    write!(output, "Just some\nRandom Words").expect("Failed to write to file");

    // Open the file and if everything is ok unwrap returns the file
    // and if not panic! triggers an error (You could replace unwrap with ?)
    // Read file using buffering
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    // You can also catch specific errors
    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            _other_error => panic!("Problem opening file : {:?}", error),
        },
    };
}
