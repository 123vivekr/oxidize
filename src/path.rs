use std::collections::HashSet;
use std::env;
use std::fs::read_dir;

pub fn fetch_binaries() -> HashSet<String> {
    let path_str = env::var("PATH").unwrap();
    let mut binaries: HashSet<String> = HashSet::new();

    for path in path_str.split(':') {
        let files = match read_dir(path) {
            Ok(files) => files,
            Err(_) => continue,
        };

        for file in files {
            let file = file.unwrap();
            let name = file.file_name().into_string();
            match name {
                Ok(name) => {
                    binaries.insert(name);
                }
                Err(_) => continue,
            };
        }
    }

    binaries
}
