use crate::{tokenize, NamingConvention};

/// Convert the input string to PascalCase.
///
/// # Examples
/// ```
/// use naming_convention::conversion::to_pascal_case;
///
/// let pascal = to_pascal_case("pascalCase");
/// assert_eq!(pascal, "PascalCase");
/// ```
pub fn to_pascal_case(s: &str) -> String {
    tokenize(s)
        .iter()
        .map(|word| {
            let word_lower = word.to_lowercase();
            let mut chars = word_lower.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// Convert the input string to camelCase.
///
/// # Examples
/// ```
/// use naming_convention::conversion::to_camel_case;
///
/// let camel = to_camel_case("camel_case");
/// assert_eq!(camel, "camelCase");
/// ```
pub fn to_camel_case(s: &str) -> String {
    let parts = tokenize(s);
    if parts.is_empty() {
        return String::new();
    }
    parts
        .iter()
        .enumerate()
        .map(|(i, word)| {
            let word_lower = word.to_lowercase();
            if i != 0 {
                let mut chars = word_lower.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                }
            } else {
                word_lower
            }
        })
        .collect()
}

/// Convert the input string to snake_case.
///
/// # Examples
/// ```
/// use naming_convention::conversion::to_snake_case;
///
/// let snake = to_snake_case("snakeCase");
/// assert_eq!(snake, "snake_case");
/// ```
pub fn to_snake_case(s: &str) -> String {
    tokenize(s)
        .iter()
        .map(|word| word.to_lowercase())
        .collect::<Vec<String>>()
        .join("_")
}

/// Convert the input string to UPPER_SNAKE_CASE.
///
/// # Examples
/// ```
/// use naming_convention::conversion::to_upper_snake_case;
///
/// let upper_snake = to_upper_snake_case("upperSnakeCase");
/// assert_eq!(upper_snake, "UPPER_SNAKE_CASE");
/// ```
pub fn to_upper_snake_case(s: &str) -> String {
    tokenize(s)
        .iter()
        .map(|word| word.to_uppercase())
        .collect::<Vec<String>>()
        .join("_")
}

/// Convert the input string to kebab-case.
///
/// # Examples
/// ```
/// use naming_convention::conversion::to_kebab_case;
///
/// let kebab = to_kebab_case("kebabCase");
/// assert_eq!(kebab, "kebab-case");
/// ```
pub fn to_kebab_case(s: &str) -> String {
    tokenize(s)
        .iter()
        .map(|word| word.to_lowercase())
        .collect::<Vec<String>>()
        .join("-")
}

/// Convert the input string to the specified naming convention.
/// If the specified naming convention is `NamingConvention::Unknown`, it returns an empty string.
///
/// # Examples
/// ```
/// use naming_convention::{conversion::convert_to, NamingConvention};
///
/// let pascal = convert_to("pascalCase", NamingConvention::PascalCase);
/// assert_eq!(pascal, "PascalCase");
/// ```
pub fn convert_to(s: &str, to: NamingConvention) -> String {
    match to {
        NamingConvention::PascalCase => to_pascal_case(s),
        NamingConvention::CamelCase => to_camel_case(s),
        NamingConvention::SnakeCase => to_snake_case(s),
        NamingConvention::KebabCase => to_kebab_case(s),
        NamingConvention::UpperCamel => to_upper_snake_case(s),
        NamingConvention::Unknown => String::new(),
    }
}
