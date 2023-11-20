mod config;
mod installer;
mod path;

fn main() {
    let alternatives = config::fetch_alternatives();
    let binaries = path::fetch_binaries();

    let mut alias_strings = vec![];

    for (native_tool, rust_tool) in &alternatives {
        if binaries.contains(native_tool) {
            installer::cargo_install(rust_tool);
            alias_strings.push(format!("alias {}={}", native_tool, rust_tool));
        }
    }

    installer::set_path();
    installer::set_alias(alias_strings);
}
