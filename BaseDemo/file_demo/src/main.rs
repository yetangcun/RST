use CfgExtensionLib::rsfile;
use std::io;

fn main() {
    let contents = rsfile::rd_relativ_file("tst.txt");
    println!("file contents: {}, demo!", contents);

    let mut inputs = String::new();
    io::stdin()
    .read_line(&mut inputs)
    .expect("Failed to read line");
}
