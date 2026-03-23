//! String, date, and list cleaning utilities

pub fn parse_list(value: &str) -> Vec<String> {
    if value.starts_with('[') && value.ends_with(']') {
        value[1..value.len() - 1]
            .split(',')
            .map(|s| s.trim().trim_matches('\'').trim_matches('"').to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        value
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_list_python_literal() {
        let result = parse_list("['Action', 'Adventure']");
        assert_eq!(result, vec!["Action", "Adventure"]);
    }

    #[test]
    fn test_parse_list_csv_format() {
        let result = parse_list("Action, Adventure, Fantasy");
        assert_eq!(result, vec!["Action", "Adventure", "Fantasy"]);
    }
}
