// use proc_macro::TokenStream;
// use quote::{quote, quote_spanned};
// use syn:: {
//     parse_macro_input, DeriveInput
// };

// #[proc_macro_derive(Arguments)]
// pub fn derive_arguments(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let DeriveInput {ident, generics, ..} = input;
//     let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
//     let data = quote! {
//         impl #ident {
//             fn get(&self) -> String {
//                 self.a.clone()
//             }
//         }
//     };

//     data.into()
// }
use std::{env::args_os, vec, ffi::OsString};

#[derive(PartialEq, Debug, Clone)]
enum ArgValueType {
    String,
    Number,
    Bool,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Arg<'a> {
    id: &'a str,
    required: bool,
    value: Option<String>,
    value_type: ArgValueType,
    usage: &'a str,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Arg {
            id: "",
            required: false,
            value: None,
            value_type: ArgValueType::Bool,
            usage: "",
        }
    }
}

impl<'a> Arg<'a> {
    fn option(&self) -> String {
        format!("--{}", self.id)
    }
    fn short_option(&self) -> String {
        format!("-{}", self.id.chars().nth(0).unwrap())
    }
    fn pattern(&self) -> String {
        format!("{}, {}", self.short_option(), self.option())
    }
    fn usage_with_pattern(&self, pad: usize) -> String {
        format!("  {: <2$}{}\n", self.pattern(), self.usage, pad,)
    }
}

#[derive(Default)]
pub struct Command<'a> {
    name: &'a str,
    description: Option<&'a str>,
    args: Vec<Arg<'a>>,
}

impl<'a> Command<'a> {
    fn new(name: &'a str) -> Self {
        Command {
            name,
            args: vec![
                Arg {
                    id: "help",
                    usage: "Print help",
                    ..Arg::default()
                },
                Arg {
                    id: "version",
                    usage: "Print version",
                    ..Arg::default()
                },
            ],
            description: None,
        }
    }

    fn description(mut self, desc: &'a str) -> Self {
        self.description = Some(desc);
        self
    }

    fn args(mut self, arg: Arg<'a>) -> Self {
        self.args.push(arg);

        self
    }

    fn usage(&self) {
        let mut usage_str = if let Some(desc) = self.description {
            format!("{}\n", desc)
        } else {
            "".to_string()
        };

        usage_str.push_str(&format!("\nUsage: {} [OPTIONS]\n\nOptions:\n", self.name));

        let max_usage_len = self.args.iter().fold(0, |acc, arg| {
            let pat_len = arg.pattern().len();
            if pat_len > acc {
                pat_len
            } else {
                acc
            }
        });

        self.args.iter().for_each(|arg| {
            let pad = max_usage_len + 2;
            usage_str.push_str(&(arg.usage_with_pattern(pad)));
        });

        println!("{}", usage_str);
    }

    fn version(&self) {
        println!("{}", env!("CARGO_PKG_VERSION"))
    }

    fn parse(&mut self) {
        self._parse(args_os().into_iter().collect());
    }

    fn _parse(&mut self, os_str: Vec<OsString>) {
        let mut args_iter = self.args.iter_mut();
        let mut index = 0;
        let is_option = |opt: &str| opt.chars().nth(0).unwrap() == '-';

        while index < os_str.len() {
            let opt_str = os_str[index].clone().into_string().unwrap();
            if is_option(&opt_str) {
                let arg = args_iter.find(|arg| arg.option() == opt_str || arg.short_option() == opt_str);

                if let Some(_arg) = arg {
                    if index < os_str.len() - 1 {
                        let value = os_str[index + 1].clone().into_string().unwrap();
                        if !is_option(&value) {
                            _arg.value = Some(value);
                        }
                    }

                    match _arg.value {
                        None => index += 1,
                        _ => index += 2
                    }

                    continue;
                }
            } 
                
            panic!("Unknown option {}", opt_str);
        }
    }
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;

    use crate::{Arg, ArgValueType, Command};

    fn create<'a>() -> Command<'a> {
        Command::new("commander").description("A cli tools builder")
    }

    #[test]
    fn new_Command() {
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
    fn unknown_option_foo(){
        let mut cmd = create();
        cmd._parse(vec![OsString::from("--foo")]);
    }

    #[test]
    #[should_panic(expected = "Unknown option --bar")]
    fn unknown_option_bar(){
        let mut cmd = create().args( Arg {
            id: "file",
            value_type: ArgValueType::String,
            usage: "Search file path",
            ..Arg::default()
        });
        cmd._parse(vec![OsString::from("--file"), OsString::from("--bar")]);
    }

    #[test]
    #[should_panic(expected = "Unknown option --baz")]
    fn unknown_option_baz(){
        let mut cmd = create().args( Arg {
            id: "file",
            value_type: ArgValueType::String,
            usage: "Search file path",
            ..Arg::default()
        });
        cmd._parse(vec![OsString::from("--file"), OsString::from("/root") ,OsString::from("--baz")]);
    }

    #[test]
    #[should_panic(expected = "Unknown option -b")]
    fn unknown_short_option(){
        let mut cmd = create().args( Arg {
            id: "file",
            value_type: ArgValueType::String,
            usage: "Search file path",
            ..Arg::default()
        });
        cmd._parse(vec![OsString::from("-f"), OsString::from("-b")]);
    }
}
