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

#[derive(PartialEq, Debug)]
enum ArgValueType {
    String,
    Number,
    Bool
}
#[derive(Debug, PartialEq)]
pub struct Arg<'a> {
    id: &'a str,
    required: bool,
    value: Option<String>,
    value_type: ArgValueType,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Arg {
            id: "",
            required: false,
            value: None,
            value_type: ArgValueType::String
        }
    }
}

// impl<'a> Arg<'a> {
//     fn get_required(&self) -> bool {
//         match self.required {
//             Some(required) => { required},
//             None => false
//         }
//     }
// }

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
            args: vec![Arg {id: "help", ..Arg::default()}],
            description: None
        }
    }

    fn description(mut self, desc: &'a str) -> Self  {
        self.description = Some(desc);
        self
    }

    fn args(mut self, arg: Arg<'a>) -> Self  {
        self.args.push(arg);

        self
    }
}

#[cfg(test)]
mod test {
    use crate::{Arg, Command};

    fn create<'a>() -> Command<'a> {
        Command::new("commander").description("A cli tools builder")
    }

    #[test]
    fn new_Command() {
        let cmd = create().args(Arg {id: "file", ..Arg::default()});
        assert_eq!(cmd.name, "commander");
        assert_eq!(cmd.description, Some("A cli tools builder"));
        assert_eq!(cmd.args[0], Arg {id: "help", ..Arg::default()});
        assert_eq!(cmd.args[1], Arg {id: "file", ..Arg::default()});
    }
}