use std::env;
use std::fs::{OpenOptions, File, read_to_string};
use std::io::{Write, Read, BufRead, BufReader};

// q directory
static Q_PATH: &str = "./q.txt";

fn main() {
    let mut args = env::args();

    // Ignore first argument (binary name)
    args.next();

    // First argument is saved into the queue
    if let Some(mut arg) = args.next() {
        // Open q file
        arg = arg + "\n";

        let mut q = OpenOptions::new().create(true).write(true).append(true).open(Q_PATH).expect("Open failed");
        q.write_all(arg.as_bytes()).expect("Write failed");

        println!("'{}' pushed successfully into q.", arg);
    }
    else {
        let mut q = OpenOptions::new().read(true).write(true).open(Q_PATH).expect("Open failed");

        // Pick line
        let line = BufReader::new(&q).lines().next().unwrap().expect("Reading failed");
        println!("{}", line);

        // Remove line
        let mut content = read_to_string(Q_PATH).expect("Error opening q");
        q.read_to_string(&mut content).expect("Error reading q.");
        let content = content.replacen((line + "\n").as_str(), "", 1);
        let mut q = File::create(Q_PATH).expect("Creation failed");
        q.write_all(content.as_bytes()).unwrap();
    }
}
