use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = flatten::determine_path(args.get(1))?;
    let name = path.to_str().unwrap();
    println!("flattening {}\nwould you like to proceed? Y/n", name);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.contains("n") || input.contains("N") {
        return Ok(());
    }
    flatten::flatten_old(&path)?;
    println!("success!");

    Ok(())
}
