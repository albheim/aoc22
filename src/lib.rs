/// Assumes the input is "nice", just made to work good enough.
pub fn split_line<'a>(string: &'a str, pattern: &str) -> Result<Vec<&'a str>, &'static str> {
    let mut results = vec![];
    let mut last_idx = 0;

    let mut first = true;
    for pat in pattern.split("{}") {
        if pat.chars().count() == 0 && last_idx != 0 {
            results.push(&string[last_idx..]);
            continue
        }
        match string.get(last_idx..).and_then(|s| s.find(pat)) {
            Some(idx) => {
                let idx = last_idx + idx;
                if !first {
                    results.push(&string[last_idx..idx]);
                }
                last_idx = idx + pat.chars().count();
                first = false;
            },
            None => return Err("Could not match pattern to string."),
        }
    }

    Ok(results) 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_edges() {
        let result = split_line("@23x", "{}@{}x{}").unwrap();
        assert_eq!(result, vec!["", "23", ""]);
    }

    #[test]
    fn test_full_edges() {
        let result = split_line("asdf@23xqwer", "{}@{}x{}").unwrap();
        assert_eq!(result, vec!["asdf", "23", "qwer"]);
    }

    #[test]
    fn test_more_complex() {
        let result = split_line("#1 @ 23,45: 43x56", "#{} @ {},{}: {}x{}").unwrap();
        assert_eq!(result, vec!["1", "23", "45", "43", "56"]);
    }
}
