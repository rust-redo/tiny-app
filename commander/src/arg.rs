#[derive(PartialEq, Debug, Clone)]
pub enum ArgValueType {
    String,
    Number,
    Bool,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Arg<'a> {
    pub id: &'a str,
    pub required: bool,
    pub value: Option<String>,
    pub value_type: ArgValueType,
    pub usage: &'a str,
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
    pub fn option(&self) -> String {
        format!("--{}", self.id)
    }
    pub fn short_option(&self) -> String {
        format!("-{}", self.id.chars().nth(0).unwrap())
    }
    pub fn pattern(&self) -> String {
        format!("{}, {}", self.short_option(), self.option())
    }
    pub fn usage_with_pattern(&self, pad: usize) -> String {
        format!("  {: <2$}{}\n", self.pattern(), self.usage, pad,)
    }
}