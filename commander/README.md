# commander

`commander` is a Rust lib that helps developers customize cli tools' commands. 


## Modules

```shell
/src
  |-- arg.rs // Arg implementation
  |-- command.rs // Command implementation
  |-- lib.rs // module entry with some test cases
  |-- main.rs // examples
```

## Usage

#### Show command usage

```shell
$ cargo run -- --help

A cli tools builder

Usage: commander [OPTIONS]

Options:
  -h, --help           Print help
  -v, --version        Print version
  -f, --file <string>  Search file path
```

#### Show command version

```shell
$ cargo run -- --version

0.1.0
```

#### Catch `--file`

```shell
$ cargo run -- --file /root

get --file value: /root
```

### Usage with `struct`

```rust
use commander::{arg::{Arg, ArgValueType}, command::Command};

fn main() {
    let mut cmd = Command::new("commander").description("A cli tools builder").args(Arg {
        id: "file",
        value_type: ArgValueType::String,
        usage: "Search file path",
        ..Arg::default()
    });

    cmd.parse();
    
    if *cmd.args_value::<bool>("help").unwrap_or(&false) {
        /*  output help usage */
        cmd.usage();
    } else if *cmd.args_value::<bool>("version").unwrap_or(&false) {
        /* output version info */
        cmd.version();
    } else if let Some(file) = cmd.args_value::<String>("file") {
        /* output file path info */
        println!("get --file value: {}", file);
    }

}
```

### Usage with `macro` 

// TODO


## References

1. [macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
2. [args_os](https://doc.rust-lang.org/std/env/fn.args_os.html)
3. [clap](https://github.com/clap-rs/clap)