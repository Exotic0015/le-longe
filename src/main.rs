use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn main() {
    let _filename: &str = &(thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect::<String>()
        + ".txt");
    println!("Started..");
    if Path::new(_filename).exists() {
        std::fs::remove_file(_filename).unwrap();
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .append(true)
        .open(_filename)
        .unwrap();

    loop {
        let random_bytes: Vec<u8> = (0..1024).map(|_| rand::random::<u8>()).collect();
        file.write_all(&random_bytes).unwrap();
    }
}
