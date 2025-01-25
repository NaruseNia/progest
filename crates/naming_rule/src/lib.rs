use regex::Regex;

/// Tokenize the input string.
///
/// # Examples
/// ```
/// use naming_rule::tokenize;
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

pub mod convert {
    use crate::tokenize;

    /// Convert the input string to PascalCase.
    ///
    /// # Examples
    /// ```
    /// use naming_rule::convert::to_pascal_case;
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
    /// use naming_rule::convert::to_camel_case;
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
    /// use naming_rule::convert::to_snake_case;
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
    /// use naming_rule::convert::to_upper_snake_case;
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
    /// use naming_rule::convert::to_kebab_case;
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
        assert_eq!(
            convert::to_snake_case("thisIsHelloWorld-Hello_World Hello world"),
            "this_is_hello_world_hello_world_hello_world"
        );
        assert_eq!(
            convert::to_kebab_case("thisIsHelloWorld-Hello_World Hello world"),
            "this-is-hello-world-hello-world-hello-world"
        );
        assert_eq!(
            convert::to_camel_case("thisIsHelloWorld-Hello_World Hello world"),
            "thisIsHelloWorldHelloWorldHelloWorld"
        );
        assert_eq!(
            convert::to_upper_snake_case("thisIsHelloWorld-Hello_World Hello world"),
            "THIS_IS_HELLO_WORLD_HELLO_WORLD_HELLO_WORLD"
        );
        assert_eq!(
            convert::to_pascal_case("thisIsHelloWorld-Hello_World Hello world"),
            "ThisIsHelloWorldHelloWorldHelloWorld"
        );
    }
}
