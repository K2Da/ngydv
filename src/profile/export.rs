pub fn rc(exports: Vec<(&str, &str)>, unsets: Vec<&str>, messages: Vec<&str>) -> String {
    let mut ret = String::new();
    for (name, value) in exports {
        ret.push_str(&format!("export {}={}\n", name, value));
    }

    for name in unsets {
        ret.push_str(&format!("unset {}\n", name));
    }

    for message in messages {
        ret.push_str(&format!("echo '{}'\n", message));
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc() {
        assert_eq!(
            rc(
                vec![("ABC", "aaa"), ("XYZ", "bbb")],
                vec!["FOO", "BAR"],
                vec!["text", "message"],
            ),
            r#"export ABC=aaa
export XYZ=bbb
unset FOO
unset BAR
echo 'text'
echo 'message'
"#,
        );
    }
}
