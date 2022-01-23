use std::fs::File;
use flate2::read::GzDecoder as Reader;
use flate2::write::GzEncoder as Writer;
use flate2::Compression;
use tar::Archive;

fn decompress_tarball() -> Result<(), std::io::Error>{
    let path = "archive.tar.gz";

    // encoded tarball
    let tar_gz = File::open(path)?;

    // decoded tarball
    let tar = Reader::new(tar_gz);

    // creating the Archive object to hold our data in memory
    let mut archive = Archive::new(tar);

    // finally unpacking everything into our current working dir
    archive.unpack(".")?;

    Ok(())
}

fn compress_tarball() -> Result<(), std::io::Error> {
    let tar_gz = File::create("compressed.tar.gz")?;
    let encoded = Writer::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(encoded);

    tar.append_dir_all("backup/logs", "/var/log")?;

    Ok(())
}


fn main() {
    //decompress_tarball().expect("PANIC!");
    //compress_tarball().expect(stringify!(Err()));
}
