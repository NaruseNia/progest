use regex::Regex;

/// Naming convention enum.
#[derive(Debug, PartialEq)]
pub enum NamingConvention {
    PascalCase,
    CamelCase,
    SnakeCase,
    KebabCase,
    UpperCamel,
    Unknown,
}

/// Tokenize the input string.
///
/// # Examples
/// ```
/// use naming_convention::tokenize;
///
/// let tokens = tokenize("thisIsHelloWorld-Hello_World Hello world");
/// assert_eq!(tokens, vec!["this", "Is", "Hello", "World", "Hello", "World", "Hello", "world"]);
/// ```
pub fn tokenize(input: &str) -> Vec<String> {
    let re = Regex::new(r"([A-Z]{2,}|[A-Z][a-z]*|[a-z]+)").unwrap();
    let parts: Vec<String> = re
        .find_iter(input)
        .map(|m| m.as_str().to_string())
        .collect();

    parts
}

/// Guess the naming convention of the input string.
/// If the input string does not match any naming convention, it returns `NamingConvention::Unknown`.
///
/// # Examples
/// ```
/// use naming_convention::{guess_naming_convention, NamingConvention};
///
/// let pascal = guess_naming_convention("PascalCase");
/// assert_eq!(pascal, NamingConvention::PascalCase);
///
/// let camel = guess_naming_convention("camelCase");
/// assert_eq!(camel, NamingConvention::CamelCase);
///
/// let snake = guess_naming_convention("snake_case");
/// assert_eq!(snake, NamingConvention::SnakeCase);
///
/// let kebab = guess_naming_convention("kebab-case");
/// assert_eq!(kebab, NamingConvention::KebabCase);
///
/// let upper_camel = guess_naming_convention("UPPER_CAMEL");
/// assert_eq!(upper_camel, NamingConvention::UpperCamel);
///
/// let unknown = guess_naming_convention("this_Is-UnknownCase");
/// assert_eq!(unknown, NamingConvention::Unknown);
/// ```
pub fn guess_naming_convention(s: &str) -> NamingConvention {
    let pascal_case_re = Regex::new(r"^[A-Z][a-z]*(?:[A-Z][a-z]*)*$").unwrap();
    if pascal_case_re.is_match(s) {
        return NamingConvention::PascalCase;
    }

    let camel_case_re = Regex::new(r"^[a-z][a-z]*(?:[A-Z][a-z]*)*$").unwrap();
    if camel_case_re.is_match(s) {
        return NamingConvention::CamelCase;
    }

    let snake_case_re = Regex::new(r"^[a-z]+(?:_[a-z]+)*$").unwrap();
    if snake_case_re.is_match(s) {
        return NamingConvention::SnakeCase;
    }

    let kebab_case_re = Regex::new(r"^[a-z]+(?:-[a-z]+)*$").unwrap();
    if kebab_case_re.is_match(s) {
        return NamingConvention::KebabCase;
    }

    let upper_camel_re = Regex::new(r"^[A-Z]+(?:_[A-Z]+)*$").unwrap();
    if upper_camel_re.is_match(s) {
        return NamingConvention::UpperCamel;
    }

    // 上記のいずれにも当てはまらない場合
    NamingConvention::Unknown
}

pub mod convert {
    use crate::tokenize;

    /// Convert the input string to PascalCase.
    ///
    /// # Examples
    /// ```
    /// use naming_convention::convert::to_pascal_case;
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
    /// use naming_convention::convert::to_camel_case;
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
    /// use naming_convention::convert::to_snake_case;
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
    /// use naming_convention::convert::to_upper_snake_case;
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
    /// use naming_convention::convert::to_kebab_case;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==== snake_case ==== //
    #[test]
    fn snake_to_upper_snake() {
        assert_eq!(convert::to_upper_snake_case("snake_case"), "SNAKE_CASE");
    }

    #[test]
    fn snake_to_kebab() {
        assert_eq!(convert::to_kebab_case("snake_case"), "snake-case");
    }

    #[test]
    fn snake_to_camel() {
        assert_eq!(convert::to_camel_case("snake_case"), "snakeCase");
    }

    #[test]
    fn snake_to_pascal() {
        assert_eq!(convert::to_pascal_case("snake_case"), "SnakeCase");
    }

    // ==== UPPER_SNAKE_CASE ==== //
    #[test]
    fn upper_snake_to_snake() {
        assert_eq!(
            convert::to_snake_case("UPPER_SNAKE_CASE"),
            "upper_snake_case"
        );
    }

    #[test]
    fn upper_snake_to_kebab() {
        assert_eq!(
            convert::to_kebab_case("UPPER_SNAKE_CASE"),
            "upper-snake-case"
        );
    }

    #[test]
    fn upper_snake_to_camel() {
        assert_eq!(convert::to_camel_case("UPPER_SNAKE_CASE"), "upperSnakeCase");
    }

    #[test]
    fn upper_snake_to_pascal() {
        assert_eq!(
            convert::to_pascal_case("UPPER_SNAKE_CASE"),
            "UpperSnakeCase"
        );
    }

    // ==== kebab-case ==== //
    #[test]
    fn kebab_to_snake() {
        assert_eq!(convert::to_snake_case("kebab-case"), "kebab_case");
    }

    #[test]
    fn kebab_to_upper_snake() {
        assert_eq!(convert::to_upper_snake_case("kebab-case"), "KEBAB_CASE");
    }

    #[test]
    fn kebab_to_camel() {
        assert_eq!(convert::to_camel_case("kebab-case"), "kebabCase");
    }

    #[test]
    fn kebab_to_pascal() {
        assert_eq!(convert::to_pascal_case("kebab-case"), "KebabCase");
    }

    // ==== camelCase ==== //
    #[test]
    fn camel_to_snake() {
        assert_eq!(convert::to_snake_case("camelCase"), "camel_case");
    }

    #[test]
    fn camel_to_upper_snake() {
        assert_eq!(convert::to_upper_snake_case("camelCase"), "CAMEL_CASE");
    }

    #[test]
    fn camel_to_kebab() {
        assert_eq!(convert::to_kebab_case("camelCase"), "camel-case");
    }

    #[test]
    fn camel_to_pascal() {
        assert_eq!(convert::to_pascal_case("camelCase"), "CamelCase");
    }

    // ==== PascalCase ==== //
    #[test]
    fn pascal_to_snake() {
        assert_eq!(convert::to_snake_case("PascalCase"), "pascal_case");
    }

    #[test]
    fn pascal_to_upper_snake() {
        assert_eq!(convert::to_upper_snake_case("PascalCase"), "PASCAL_CASE");
    }

    #[test]
    fn pascal_to_kebab() {
        assert_eq!(convert::to_kebab_case("PascalCase"), "pascal-case");
    }

    #[test]
    fn pascal_to_camel() {
        assert_eq!(convert::to_camel_case("PascalCase"), "pascalCase");
    }

    // ==== tokenize ==== //
    #[test]
    fn tokenize_complex() {
        assert_eq!(
            tokenize("thisIsHelloWorld-Hello_World Hello world"),
            vec!["this", "Is", "Hello", "World", "Hello", "World", "Hello", "world"]
        );
    }

    // ==== Complex pattern ==== //
    #[test]
    fn complex_pattern_conversion() {
        let input = "thisIsHelloWorld-Hello_World Hello world";
        assert_eq!(
            convert::to_snake_case(input),
            "this_is_hello_world_hello_world_hello_world"
        );
        assert_eq!(
            convert::to_kebab_case(input),
            "this-is-hello-world-hello-world-hello-world"
        );
        assert_eq!(
            convert::to_camel_case(input),
            "thisIsHelloWorldHelloWorldHelloWorld"
        );
        assert_eq!(
            convert::to_upper_snake_case(input),
            "THIS_IS_HELLO_WORLD_HELLO_WORLD_HELLO_WORLD"
        );
        assert_eq!(
            convert::to_pascal_case(input),
            "ThisIsHelloWorldHelloWorldHelloWorld"
        );
    }

    // ==== guess_naming_convention ==== //
    #[test]
    fn guess_pascal_case() {
        assert_eq!(
            guess_naming_convention("PascalCase"),
            NamingConvention::PascalCase
        );
    }

    #[test]
    fn guess_camel_case() {
        assert_eq!(
            guess_naming_convention("camelCase"),
            NamingConvention::CamelCase
        );
    }

    #[test]
    fn guess_snake_case() {
        assert_eq!(
            guess_naming_convention("snake_case"),
            NamingConvention::SnakeCase
        );
    }

    #[test]
    fn guess_kebab_case() {
        assert_eq!(
            guess_naming_convention("kebab-case"),
            NamingConvention::KebabCase
        );
    }

    #[test]
    fn guess_upper_camel() {
        assert_eq!(
            guess_naming_convention("UPPER_CAMEL"),
            NamingConvention::UpperCamel
        );
    }

    #[test]
    fn guess_unknown() {
        assert_eq!(
            guess_naming_convention("this_Is-UnknownCase"),
            NamingConvention::Unknown
        );
    }
}
