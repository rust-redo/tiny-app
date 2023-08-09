#[macro_export]
macro_rules! arg {
    (--$name:ident <$type:ident> $usage:literal) => {
        $crate::Arg {
            id: stringify!($name),
            value_type: match stringify!($type) {
                "string" => $crate::ArgValueType::String,
                "number" => $crate::ArgValueType::Number,
                _ => $crate::ArgValueType::Bool,
            },
            usage: $usage,
            ..$crate::Arg::default()
        }
    };
}
