use memmap::Mmap;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

// Generally it is good practice to not read AND write from a same file
// to avoid this we can use the same_file crate to test for equality
fn in_out() -> Result<(), Error> {
    let path = "lines.txt";

    let mut output = File::create(path)?;
    write!(output, "My hovercraft is full of eels!")?; // arguments are an input file and a string slice

    let input = File::open(path)?; // opening the file with path variable
    let buffered = BufReader::new(input); // creating a buffer to be able to read out text file in

    for line in buffered.lines() {
        println!("{}", line?)
    }

    Ok(())

}

// gain the ability to create a memorymap of the file, letting us index into a slice
// verse seeking through the file as std does. Avoid writing to the file at the same time
// can cause race conditions
fn mem_map() -> Result<(), Error> {
    let file = File::open("lines.txt")?;
    let map = unsafe { Mmap::map(&file)? };

    let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
    println!("{:?}", &map[3..13] == b"hovercraft");

    let random_bytes: Vec<u8> = random_indexes.iter()
        .map(|&idx| map[idx])
        .collect();
    println!("{:?}", &random_bytes[..] == b"My loaf!");
    Ok(())
}

fn main() {
    in_out();
    mem_map();
}
