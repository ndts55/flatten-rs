use std::{self, env, fs, io, path};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
}

fn main() {
    let args = Args::from_args();
    let res = flatten(&args.path);
    println!("{:?}", res);
}

fn flatten(dir: &path::Path) -> io::Result<()> {
    if dir.is_dir() {
        env::set_current_dir(dir).unwrap();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let mut path = entry.path();
            let path_name = path.file_name().unwrap().to_str().unwrap().to_owned();
            if path.is_dir() {
                env::set_current_dir(&path).unwrap();
                // move children outside, prepend own name
                for child in fs::read_dir(&path)? {
                    let child = child?;
                    let f_n = child.file_name();
                    let name = f_n.to_str().unwrap();
                    let new_name = path_name.clone() + " - " + name;
                    println!("{}", new_name);
                    fs::rename(name, "../".to_string() + &new_name)?;
                }
                path.pop();
                env::set_current_dir(path).unwrap();
            }
        }
    }

    Ok(())
}
