use inflector::cases::snakecase::to_snake_case;
use regex::Regex;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    directory: PathBuf,
}

fn main() {
    let opt: Opt = Opt::from_args();
    let path = Path::new(&opt.directory);
    rename(path);
}

fn rename(path: &Path) {
    for entry in path.read_dir().expect("failed to read") {
        if let Ok(entry) = entry {
            let mut full_path = entry.path();

            let mut full_name = full_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_lowercase();

            let first_alpha_numeric = Regex::new(r"[A-Za-z0-9]")
                .unwrap()
                .find(full_name.as_str())
                .unwrap()
                .start();

            let name = full_name.split_off(first_alpha_numeric);
            let snake_case_name = to_snake_case(&name);

            let extension = full_path.extension().unwrap();

            let new_name = format!(
                "{}{}.{}",
                full_name,
                snake_case_name,
                &extension.to_str().unwrap()
            );
            println!("{}", new_name);
            full_path.set_file_name(new_name);
            std::fs::rename(entry.path(), full_path).unwrap();
        }
    }
}
