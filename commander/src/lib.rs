mod arg;
mod command;

#[cfg(test)]
mod test {
    use std::ffi::OsString;

    use crate::{
        arg::{Arg, ArgValueType},
        command::Command,
    };

    fn create<'a>() -> Command<'a> {
        Command::new("commander").description("A cli tools builder")
    }

    #[test]
    fn new_command() {
        let file_arg = Arg {
            id: "file",
            value_type: ArgValueType::String,
            usage: "Search file path",
            ..Arg::default()
        };
        let cmd = create().args(file_arg.clone());
        assert_eq!(cmd.name, "commander");
        assert_eq!(cmd.description, Some("A cli tools builder"));
        assert_eq!(
            cmd.args[0].id,
            "help"
        );
        assert_eq!(
            cmd.args[1].id,
            "version"
        );
        assert_eq!(cmd.args[2].id, "file");
        cmd.usage();
    }

    #[test]
    #[should_panic(expected = "Unknown option --foo")]
    fn unknown_option_foo() {
        let mut cmd = create();
        cmd._parse(vec![OsString::from("--foo")]);
    }

    #[test]
    #[should_panic(expected = "Unknown option --bar")]
    fn unknown_option_bar() {
        let mut cmd = create().args(Arg {
            id: "file",
            value_type: ArgValueType::String,
            usage: "Search file path",
            ..Arg::default()
        });
        cmd._parse(vec![OsString::from("--file"), OsString::from("--bar")]);
    }

    #[test]
    #[should_panic(expected = "Unknown option --baz")]
    fn unknown_option_baz() {
        let mut cmd = create().args(Arg {
            id: "file",
            value_type: ArgValueType::String,
            usage: "Search file path",
            ..Arg::default()
        });
        cmd._parse(vec![
            OsString::from("--file"),
            OsString::from("/root"),
            OsString::from("--baz"),
        ]);
    }

    #[test]
    #[should_panic(expected = "Unknown option -b")]
    fn unknown_short_option() {
        let mut cmd = create().args(Arg {
            id: "file",
            value_type: ArgValueType::String,
            usage: "Search file path",
            ..Arg::default()
        });
        cmd._parse(vec![OsString::from("-f"), OsString::from("-b")]);
    }

    #[test]
    fn should_parse_string() {
        let mut cmd = create().args(Arg {
            id: "file",
            value_type: ArgValueType::String,
            usage: "Search file path",
            ..Arg::default()
        });

        cmd._parse(vec![OsString::from("-f"), OsString::from("/root")]);

        let file_path = cmd.args_value::<String>("file");

        assert_eq!(file_path, Some(&"/root".to_string()));
    }

    #[test]
    fn should_parse_number() {
        let mut cmd = create().args(Arg {
            id: "port",
            value_type: ArgValueType::Number,
            usage: "TCP port",
            ..Arg::default()
        });

        cmd._parse(vec![OsString::from("-p"), OsString::from("8080")]);

        let port = cmd.args_value::<i32>("port");

        assert_eq!(port, Some(&8080));
    }

    #[test]
    fn should_parse_bool() {
        let mut cmd = create();

        cmd._parse(vec![OsString::from("-h")]);

        let help = cmd.args_value::<bool>("help");

        assert_eq!(help, Some(&true));
    }
}
