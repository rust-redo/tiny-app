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
            cmd.args[0],
            Arg {
                id: "help",
                usage: "Print help",
                ..Arg::default()
            }
        );
        assert_eq!(
            cmd.args[1],
            Arg {
                id: "version",
                usage: "Print version",
                ..Arg::default()
            }
        );
        assert_eq!(cmd.args[2], file_arg);
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
}
