use std::env;
use std::fs::{OpenOptions};
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
        arg.push_str("\n");

        let mut q = OpenOptions::new().create(true).write(true).append(true).open(Q_PATH).expect("Open failed");
        q.write_all(arg.as_bytes()).expect("Write failed");

        println!("'{}' pushed successfully into q.", arg);
    }
    else {
        let mut q = OpenOptions::new().read(true).write(true).append(false).open(Q_PATH).expect("Open failed");

        // Pick line
        let line = BufReader::new(&q).lines().next().unwrap().expect("Reading failed");
        println!("{}", line);

        // Remove line
        let mut content = String::new();
        q.read_to_string(&mut content).expect("Error reading q.");
        let content = content.replacen(line.as_str(), "", 1);
        println!("{}", content);
        q.write_all(content.as_bytes()).unwrap();
    }
}
