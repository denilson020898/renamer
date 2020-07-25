use inflector::cases::snakecase::to_snake_case;
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
            let mut full_name = entry.path();

            let name = full_name
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_lowercase();
            let snake_case_name = to_snake_case(&name);
            let extension = full_name.extension().unwrap();
            let new_name = format!("{}.{}", snake_case_name, &extension.to_str().unwrap());
            full_name.set_file_name(new_name);
            std::fs::rename(entry.path(), full_name).unwrap();
        }
    }
}
