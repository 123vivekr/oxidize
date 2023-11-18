mod config;
mod installer;
mod path;

fn main() {
    let alternatives = config::fetch_alternatives();
    let binaries = path::fetch_binaries();

    for (native_tool, rust_tool) in &alternatives {
        if binaries.contains(native_tool) {
            installer::cargo_install(rust_tool);
        }
    }

    installer::set_path();
}
