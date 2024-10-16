pub fn to_kebab_case(input: &str) -> String {
    // convert to lowercase and replace spaces with hyphens
    input
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

pub fn to_snake_case(input: &str) -> String {
    // convert to lowercase and replace spaces with underscores
    input
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_kebab_case() {
        assert_eq!(to_kebab_case("Monea Devnet L2"), "monea-devnet-l2");
        assert_eq!(to_kebab_case("Hello World"), "hello-world");
        assert_eq!(to_kebab_case("camelCase"), "camelcase");
        assert_eq!(to_kebab_case("PascalCase"), "pascalcase");
        assert_eq!(to_kebab_case("snake_case"), "snake-case");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("Monea Devnet L2"), "monea_devnet_l2");
        assert_eq!(to_snake_case("Hello World"), "hello_world");
        assert_eq!(to_snake_case("camelCase"), "camel_case");
        assert_eq!(to_snake_case("PascalCase"), "pascal_case");
        assert_eq!(to_snake_case("snake_case"), "snake_case");
    }
}
