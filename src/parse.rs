/// Parse a command from the input string. Input string is space seperated (unless
/// enclosed in quotes), returns a vec of parsed strings. The first string should
/// be intpreted as the command, all subsequent string as arguments.
pub fn parse_command(input: &str) -> Result<Vec<String>, String> {
    let input = input.trim();

    // let mut count = 0;
    let mut command = vec![];

    let mut to_parse = input;
    let mut iter = to_parse.char_indices();

    while let Some((idx, ch)) = iter.next() {
        if ch.is_whitespace() {
            continue;
        }

        let (remaining, parsed) = parse_space_seperated_chunk(&to_parse[idx..])?;
        // dbg!(&parsed);
        // dbg!(&remaining);

        to_parse = remaining;
        iter = to_parse.char_indices();

        command.push(parsed);

        // count += 1;
        // if count > 2 {
        //     break;
        // }
    }

    Ok(command)
}

/// Parse the first space-seperated chunk of the input. If a section is in quotes, spaces
/// inside the should not be treated as seperators. Return tuple of remaining
/// unparsed input and parsed chunk
fn parse_space_seperated_chunk(input: &str) -> Result<(&str, String), String> {
    let mut acc = String::new();
    let mut iter = input.char_indices();

    while let Some((idx, ch)) = iter.next() {
        if ch.is_whitespace() {
            return Ok((&input[idx..], acc));
        }

        if ch == '"' || ch == '\'' {
            let (remaining, parsed) = parse_quoted(&input[idx..])?;

            acc += parsed;

            iter = remaining.char_indices();
        } else {
            acc.push(ch);
        }
    }

    Ok((&input[input.len()..], acc))
}

/// Attempt to parse the first quoted string from a &str.  Returns Err if input is empty or
/// if the first character of the input is not `"` or `'`, or no matching quote
/// is found. Returns tuple of remaining unparsed input, and parsed string EXCLUDING quotes.
fn parse_quoted(input: &str) -> Result<(&str, &str), String> {
    let mut chars = input.char_indices();
    let (_idx, quote) = chars.next().ok_or_else(|| {
        format!("expected quote, found nothing at position 0 when trying to parse a quoted string")
    })?;

    if quote != '"' && quote != '\'' {
        return Err(format!("Expected a single or double quote, found {quote} at position 0 when trying to parse a quoted string"));
    }

    while let Some((idx, ch)) = chars.next() {
        if ch == quote {
            let qlen = quote.len_utf8();

            return Ok((&input[idx + qlen..], &input[qlen..idx]));
        }
    }

    Err(format!(
        "could not find closing quotes when trying to parse `{input}`, expected `{quote}`"
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_command() {
        for (input, expected) in [
            (r#"ls"#, Ok(vec![String::from("ls")])),
            (
                r#"ls -lha"#,
                Ok(vec![String::from("ls"), String::from("-lha")]),
            ),
            (
                r#"ls -lha /sys"#,
                Ok(vec![
                    String::from("ls"),
                    String::from("-lha"),
                    String::from("/sys"),
                ]),
            ),
            (
                r#"echo  extra   spaces  will    be    removed"#,
                Ok(vec![
                    String::from("echo"),
                    String::from("extra"),
                    String::from("spaces"),
                    String::from("will"),
                    String::from("be"),
                    String::from("removed"),
                ]),
            ),
            (
                "echo \"but   not  if    they're    in    quotes\"\n",
                // echo "but   not  if    they're    in    quotes"
                Ok(vec![
                    String::from("echo"),
                    String::from("but   not  if    they're    in    quotes"),
                ]),
            ),
        ] {
            let actual = parse_command(input);
            assert_eq!(
                actual, expected,
                "got left when expecting right from input `{input:?}`"
            );
        }
    }

    #[test]
    fn test_parse_space_seperated_chunk() {
        for (input, expected) in [
            ("ls", Ok(("", String::from("ls")))),
            ("ls -l", Ok((" -l", String::from("ls")))),
            ("ls -lha", Ok((" -lha", String::from("ls")))),
            ("ls -l /sys", Ok((" -l /sys", String::from("ls")))),
        ] {
            let actual = parse_space_seperated_chunk(input);
            assert_eq!(
                actual, expected,
                "got left when expecting right from input `{input:?}`"
            );
        }

        for (input, expected) in [
            (r#""ls""#, String::from("ls")),
            (
                r#""Movie name with spaces".mkv"#,
                String::from("Movie name with spaces.mkv"),
            ),
            (r#"'ls'"#, String::from("ls")),
            (
                r#"'Movie name with spaces'.mkv"#,
                String::from("Movie name with spaces.mkv"),
            ),
        ] {
            let actual = parse_space_seperated_chunk(input);
            let expected = Ok(("", expected));
            assert_eq!(
                actual, expected,
                "got left when expecting right from input `{input:?}`"
            );
        }
    }

    #[test]
    fn test_parse_quoted() {
        for (input, expected) in [
            (r#""ls""#, Ok(("", "ls"))),
            (
                r#""Movie name with spaces".mkv"#,
                Ok((".mkv", "Movie name with spaces")),
            ),
            (r#"'ls'"#, Ok(("", "ls"))),
            (
                r#"'Movie name with spaces'.mkv"#,
                Ok((".mkv", "Movie name with spaces")),
            ),
        ] {
            let actual = parse_quoted(input);
            assert_eq!(
                actual, expected,
                "got left when expecting right from input `{input:?}`"
            );
        }

        for input in [r#"Missing Opening Quotes""#, r#""Missing Closing Quotes"#] {
            let actual = parse_quoted(input);
            assert!(actual.is_err());
        }
    }
}
