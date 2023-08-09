use commander::{
    arg::{Arg, ArgValueType},
    command::Command,
};

fn main() {
    let mut cmd = Command::new("commander")
        .description("A cli tools builder")
        .args(Arg {
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
