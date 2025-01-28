pub mod conversion;

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
/// let tokens = tokenize("00thisIsHelloWorld-Hello_World Hello world 02");
/// assert_eq!(tokens, vec!["00", "this", "Is", "Hello", "World", "Hello", "World", "Hello", "world", "02"]);
/// ```
pub fn tokenize(input: &str) -> Vec<String> {
    let re = Regex::new(r"([A-Z]{2,}|[A-Z][a-z]*|[a-z]+|\d+)").unwrap();
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
    let pascal_case_re = Regex::new(r"^[A-Z0-9][a-z0-9]*(?:[A-Z0-9][a-z0-9]*)*$").unwrap();
    if pascal_case_re.is_match(s) {
        return NamingConvention::PascalCase;
    }

    let camel_case_re = Regex::new(r"^[a-z0-9][a-z0-9]*(?:[A-Z][a-z0-9]*)*$").unwrap();
    if camel_case_re.is_match(s) {
        return NamingConvention::CamelCase;
    }

    let snake_case_re = Regex::new(r"^[a-z0-9]+(?:_[a-z0-9]+)*$").unwrap();
    if snake_case_re.is_match(s) {
        return NamingConvention::SnakeCase;
    }

    let kebab_case_re = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
    if kebab_case_re.is_match(s) {
        return NamingConvention::KebabCase;
    }

    let upper_camel_re = Regex::new(r"^[A-Z0-9]+(?:_[A-Z0-9]+)*$").unwrap();
    if upper_camel_re.is_match(s) {
        return NamingConvention::UpperCamel;
    }

    // 上記のいずれにも当てはまらない場合
    NamingConvention::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==== snake_case ==== //
    #[test]
    fn snake_to_upper_snake() {
        assert_eq!(
            conversion::to_upper_snake_case("snake_case_00"),
            "SNAKE_CASE_00"
        );
    }

    #[test]
    fn snake_to_kebab() {
        assert_eq!(conversion::to_kebab_case("snake_case_00"), "snake-case-00");
    }

    #[test]
    fn snake_to_camel() {
        assert_eq!(conversion::to_camel_case("snake_case_00"), "snakeCase00");
    }

    #[test]
    fn snake_to_pascal() {
        assert_eq!(conversion::to_pascal_case("snake_case_00"), "SnakeCase00");
    }

    // ==== UPPER_SNAKE_CASE ==== //
    #[test]
    fn upper_snake_to_snake() {
        assert_eq!(
            conversion::to_snake_case("UPPER_SNAKE_CASE_00"),
            "upper_snake_case_00"
        );
    }

    #[test]
    fn upper_snake_to_kebab() {
        assert_eq!(
            conversion::to_kebab_case("UPPER_SNAKE_CASE_00"),
            "upper-snake-case-00"
        );
    }

    #[test]
    fn upper_snake_to_camel() {
        assert_eq!(
            conversion::to_camel_case("UPPER_SNAKE_CASE_00"),
            "upperSnakeCase00"
        );
    }

    #[test]
    fn upper_snake_to_pascal() {
        assert_eq!(
            conversion::to_pascal_case("UPPER_SNAKE_CASE_00"),
            "UpperSnakeCase00"
        );
    }

    // ==== kebab-case ==== //
    #[test]
    fn kebab_to_snake() {
        assert_eq!(conversion::to_snake_case("kebab-case-00"), "kebab_case_00");
    }

    #[test]
    fn kebab_to_upper_snake() {
        assert_eq!(
            conversion::to_upper_snake_case("kebab-case-00"),
            "KEBAB_CASE_00"
        );
    }

    #[test]
    fn kebab_to_camel() {
        assert_eq!(conversion::to_camel_case("kebab-case-00"), "kebabCase00");
    }

    #[test]
    fn kebab_to_pascal() {
        assert_eq!(conversion::to_pascal_case("kebab-case-00"), "KebabCase00");
    }

    // ==== camelCase ==== //
    #[test]
    fn camel_to_snake() {
        assert_eq!(conversion::to_snake_case("camelCase00"), "camel_case_00");
    }

    #[test]
    fn camel_to_upper_snake() {
        assert_eq!(
            conversion::to_upper_snake_case("camelCase00"),
            "CAMEL_CASE_00"
        );
    }

    #[test]
    fn camel_to_kebab() {
        assert_eq!(conversion::to_kebab_case("camelCase00"), "camel-case-00");
    }

    #[test]
    fn camel_to_pascal() {
        assert_eq!(conversion::to_pascal_case("camelCase00"), "CamelCase00");
    }

    // ==== PascalCase ==== //
    #[test]
    fn pascal_to_snake() {
        assert_eq!(conversion::to_snake_case("PascalCase00"), "pascal_case_00");
    }

    #[test]
    fn pascal_to_upper_snake() {
        assert_eq!(
            conversion::to_upper_snake_case("PascalCase00"),
            "PASCAL_CASE_00"
        );
    }

    #[test]
    fn pascal_to_kebab() {
        assert_eq!(conversion::to_kebab_case("PascalCase00"), "pascal-case-00");
    }

    #[test]
    fn pascal_to_camel() {
        assert_eq!(conversion::to_camel_case("PascalCase00"), "pascalCase00");
    }

    // ==== tokenize ==== //
    #[test]
    fn tokenize_complex() {
        assert_eq!(
            tokenize("00thisIsHelloWorld-Hello_World Hello world 02"),
            vec!["00", "this", "Is", "Hello", "World", "Hello", "World", "Hello", "world", "02"]
        );
    }

    // ==== Complex pattern ==== //
    #[test]
    fn complex_pattern_conversion() {
        let input = "thisIsHelloWorld-Hello_World Hello world00";
        assert_eq!(
            conversion::to_snake_case(input),
            "this_is_hello_world_hello_world_hello_world_00"
        );
        assert_eq!(
            conversion::to_kebab_case(input),
            "this-is-hello-world-hello-world-hello-world-00"
        );
        assert_eq!(
            conversion::to_camel_case(input),
            "thisIsHelloWorldHelloWorldHelloWorld00"
        );
        assert_eq!(
            conversion::to_upper_snake_case(input),
            "THIS_IS_HELLO_WORLD_HELLO_WORLD_HELLO_WORLD_00"
        );
        assert_eq!(
            conversion::to_pascal_case(input),
            "ThisIsHelloWorldHelloWorldHelloWorld00"
        );
    }

    #[test]
    fn special_pattern_conversion() {
        assert_eq!(
            conversion::to_snake_case("20000101TestName0001"),
            "20000101_test_name_0001"
        );
        assert_eq!(
            conversion::to_upper_snake_case("20000101TestName0001"),
            "20000101_TEST_NAME_0001"
        );
        assert_eq!(
            conversion::to_kebab_case("20000101TestName0001"),
            "20000101-test-name-0001"
        );
        assert_eq!(
            conversion::to_camel_case("20000101_test_name_0001"),
            "20000101TestName0001"
        );
        assert_eq!(
            conversion::to_pascal_case("20000101_test_name_0001"),
            "20000101TestName0001"
        );

        assert_eq!(
            conversion::to_snake_case("0001 This is test, is it?"),
            "0001_this_is_test_is_it"
        );
        assert_eq!(
            conversion::to_upper_snake_case("0001 This is test, is it?"),
            "0001_THIS_IS_TEST_IS_IT"
        );
        assert_eq!(
            conversion::to_kebab_case("0001 This is test, is it?"),
            "0001-this-is-test-is-it"
        );
        assert_eq!(
            conversion::to_camel_case("0001 This is test, is it?"),
            "0001ThisIsTestIsIt"
        );
        assert_eq!(
            conversion::to_pascal_case("0001 This is test, is it?"),
            "0001ThisIsTestIsIt"
        );
    }

    // ==== guess_naming_convention ==== //
    #[test]
    fn guess_pascal_case() {
        assert_eq!(
            guess_naming_convention("PascalCase00"),
            NamingConvention::PascalCase
        );
    }

    #[test]
    fn guess_camel_case() {
        assert_eq!(
            guess_naming_convention("camelCase00"),
            NamingConvention::CamelCase
        );
    }

    #[test]
    fn guess_snake_case() {
        assert_eq!(
            guess_naming_convention("snake_case_00"),
            NamingConvention::SnakeCase
        );
    }

    #[test]
    fn guess_kebab_case() {
        assert_eq!(
            guess_naming_convention("kebab-case-00"),
            NamingConvention::KebabCase
        );
    }

    #[test]
    fn guess_upper_camel() {
        assert_eq!(
            guess_naming_convention("UPPER_CAMEL_00"),
            NamingConvention::UpperCamel
        );
    }

    #[test]
    fn guess_unknown() {
        assert_eq!(
            guess_naming_convention("this_Is-UnknownCase00"),
            NamingConvention::Unknown
        );
    }

    // ==== convert_to ==== //
    #[test]
    fn convert_to_pascal_case() {
        assert_eq!(
            conversion::convert_to("pascalCase", NamingConvention::PascalCase),
            "PascalCase"
        );
    }

    #[test]
    fn convert_to_camel_case() {
        assert_eq!(
            conversion::convert_to("CamelCase", NamingConvention::CamelCase),
            "camelCase"
        );
    }

    #[test]
    fn convert_to_snake_case() {
        assert_eq!(
            conversion::convert_to("snake_case", NamingConvention::SnakeCase),
            "snake_case"
        );
    }

    #[test]
    fn convert_to_kebab_case() {
        assert_eq!(
            conversion::convert_to("kebab-case", NamingConvention::KebabCase),
            "kebab-case"
        );
    }

    #[test]
    fn convert_to_upper_camel() {
        assert_eq!(
            conversion::convert_to("UPPER_CAMEL", NamingConvention::UpperCamel),
            "UPPER_CAMEL"
        );
    }
}
