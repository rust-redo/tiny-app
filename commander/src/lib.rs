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
use std::{env::args_os, vec};

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
    fn pattern(&self) -> String {
        format!("-{}, --{}", self.id.chars().nth(0).unwrap(), self.id)
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

    // fn parse(&self) -> {
    // }
}

#[cfg(test)]
mod test {
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

        cmd.usage();
        cmd.version();
    }
}
