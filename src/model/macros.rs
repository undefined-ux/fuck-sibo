macro_rules! impl_display {
    ($struct_name:ident) => {
        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", serde_json::to_string(self).unwrap())
            }
        }
    };
}

/// 实现从json字符串解析到结构体, 结构体必须有Deserialize trait
macro_rules! impl_from_string {
    ($struct_name:ident) => {
        impl From<String> for $struct_name {
            fn from(s: String) -> Self {
                serde_json::from_str(&s).unwrap()
            }
        }
    };
}
