# Oxidize

Oxidize literally oxidizes your command line. 
Native system tools are 'replaced' with alteranatives written in Rust.

## How it works?

Just run oxidize
```
cargo run
```

Under the hood, `oxidize` installs rust alternatives (found in `alternatives.toml`) and updates the $PATH to point to `$HOME/.cargo/bin`.
