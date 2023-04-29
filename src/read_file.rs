use std::fs;
use std::io;

fn main() -> io::Result<()> {
    print_file_contents("virtual_host_example.conf")?;
    Ok(())
}

fn print_file_contents(filename: &str) -> io::Result<()> {
    let content = fs::read_to_string(filename)?;
    println!("{}", content);
    Ok(())
}
