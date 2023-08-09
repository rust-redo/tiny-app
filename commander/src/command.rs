use crate::arg::{Arg, ArgValueType};
use std::{any::Any, env::args_os, ffi::OsString, rc::Rc, vec};

#[derive(Default)]
pub struct Command<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub args: Vec<Arg<'a>>,
}

impl<'a> Command<'a> {
    /* init command's args with built-in options --help and --version */
    pub fn new(name: &'a str) -> Self {
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

    /* set command's description */
    pub fn description(mut self, desc: &'a str) -> Self {
        self.description = Some(desc);
        self
    }

    /* add command's argument */
    pub fn args(mut self, arg: Arg<'a>) -> Self {
        self.args.push(arg);

        self
    }

    /* get dynamic value after parsing args_os */
    pub fn args_value<T: Any + 'static>(&self, id: &str) -> Option<&T> {
        let arg = self.args.iter().find(|arg| arg.id == id);

        match arg {
            Some(_arg) => _arg.value::<T>(),
            None => None,
        }
    }

    /* format and print command's usage */
    pub fn usage(&self) {
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

    /* print command's version info */
    pub fn version(&self) {
        println!("{}", env!("CARGO_PKG_VERSION"))
    }

    /* parse arguments' values from args_os */
    pub fn parse(&mut self) {
        self._parse(args_os().into_iter().collect());
    }

    /* core logic of parsing values */
    pub fn _parse(&mut self, os_str: Vec<OsString>) {
        let mut args_iter = self.args.iter_mut();
        /* ignore command's name */
        let mut index = 1;
        /* valid option should start with "-" prefix */
        let is_option = |opt: &str| opt.chars().nth(0).unwrap() == '-';

        while index < os_str.len() {
            let opt_str = os_str[index].clone().into_string().unwrap();

            if is_option(&opt_str) {
                /* find the related Arg from self.args */
                let arg =
                    args_iter.find(|arg| arg.option() == opt_str || arg.short_option() == opt_str);

                if let Some(_arg) = arg {
                    /* bool value doesn't need read value after option */
                    if _arg.value_type == ArgValueType::Bool {
                        _arg.value = Some(Rc::new(true));
                    }
                    if index < os_str.len() - 1 {
                        let value = os_str[index + 1].clone().into_string().unwrap();
                        if !is_option(&value) {
                            /* covert value types from OsString */
                            _arg.value = Some(match _arg.value_type {
                                ArgValueType::Number => {
                                    Rc::new(value.trim().parse::<i32>().unwrap())
                                }
                                ArgValueType::String => Rc::new(value),
                                ArgValueType::Bool => Rc::new(value == "true"),
                            });
                        }
                    }

                    match _arg.value {
                        None => index += 1,
                        _ => index += 2,
                    }

                    continue;
                }
            }

            panic!("Unknown option {}", opt_str);
        }
    }
}
