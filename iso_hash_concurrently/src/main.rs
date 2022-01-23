use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufReader, Read, Error};
use std::path::Path;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use ring::digest::{Context, Digest, SHA256};

fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {

    // creating a reader to read our buffer
    let mut buf_reader = BufReader::new(File::open(&filepath)?);

    // crypto context
    let mut context = Context::new(&SHA256);

    // creating a buffer to hold hashed stuff inside
    let mut buffer = [0; 1024];

    // looping until we have no more data to read
    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok((context.finish(), filepath))
}

// helper function to determine if the entry passed is a file with the .iso extension
// will return a true value if this is the case, otherwise false will be returned
fn is_iso(entry: &Path) -> bool {
    match entry.extension() {
        Some(e) if e.to_string_lossy().to_lowercase() == "iso" => true,
        _ => false,
    }
}

fn main() -> Result<(), Error> {
    // creating a pool of threads, getting current number of cpu cores(threads)
    let pool = ThreadPool::new(num_cpus::get());

    // creating transmitting and recv channels
    let (tx, rx) = channel();

    // walking through each item in the directory, eventually searching for an .iso file to hash
    for entry in WalkDir::new("/home/Storage/Torrents")
        .follow_links(true)
        .into_iter() // making each object iterable
        .filter_map(|e| e.ok()) // shortens a .map().filter().map() chain
        .filter(|e| !e.path().is_dir() && is_iso(e.path())) { // if the current filepath is not a director and is an iso we will calc the hash value
            let path = entry.path().to_owned(); // turning our path from barrowed memory into OWNED data
            let tx = tx.clone(); // creating a copy out of trans channel

            // executing our digest helper function on an open thread in the thread pool
            pool.execute( move || {
                let digest = compute_digest(path);
                tx.send(digest).expect("Could not send data!");
            });
        }

    drop(tx);Before I invest the time myself and double the work: anyone aware about a new plugin for NeoVim that allows to use the recently added Tree-sitter integration to access the code structure nodes as text objects or for navigation? It would be great to get rid of all this logic with stupid p
    // stepping through our items in the recv channel and printing out the information contained within
    for t in rx.iter() {
        let (sha, path) = t?;
        println!("SHA256: {:?} Path: {:?}", sha, path);
    }
    Ok(())
}
