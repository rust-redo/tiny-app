// use std::{any::Any, i32};

use std::{any::Any, rc::Rc};

#[derive(PartialEq, Debug, Clone)]
pub enum ArgValueType {
    String,
    Number,
    Bool,
}
#[derive(Clone, Debug)]
pub struct Arg<'a> {
    pub id: &'a str,
    pub value: Option<Rc<dyn Any + 'static>>,
    pub value_type: ArgValueType,
    pub usage: &'a str,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Arg {
            id: "",
            value: None,
            value_type: ArgValueType::Bool,
            usage: "",
        }
    }
}

impl<'a> Arg<'a> {
    pub fn option(&self) -> String {
        format!("--{}", self.id)
    }
    pub fn short_option(&self) -> String {
        format!("-{}", self.id.chars().nth(0).unwrap())
    }
    pub fn pattern(&self) -> String {
        format!("{}, {}{}", self.short_option(), self.option(), match self.value_type {
            ArgValueType::Bool => "",
            ArgValueType::String => " <string>",
            ArgValueType::Number => " <number>"
        } , )
    }
    pub fn usage_with_pattern(&self, pad: usize) -> String {
        format!("  {: <2$}{}\n", self.pattern(), self.usage, pad,)
    }
    pub fn value<T>(&self) -> Option<&T>
    where T: Any + 'static
    {
        match self.value {
            Some(ref v) => {v.downcast_ref::<T>()},
            None => None
        }
    }
}
