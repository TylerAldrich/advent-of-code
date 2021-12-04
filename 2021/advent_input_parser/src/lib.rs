use std::io::BufRead;

pub fn parse_by_line<R>(mut reader: R) -> Vec<String>
where
    R: BufRead,
{
    let mut read_lines = Vec::new();
    loop {
        let mut input = String::new();

        let result = reader
            .read_line(&mut input)
            .expect("Problem reading stdin lines!");

        // This occurs when EOF has been reached
        if result == 0 {
            break;
        }

        read_lines.push(input.trim().to_string());
    }

    read_lines
}

#[cfg(test)]
mod tests {
    use crate::parse_by_line;

    // Short explanation for future tyler:
    // &input[..] turns our array of u8 into a slice, which
    // implements BufRead.
    // This is used to test parse_by_line above, which generally accepts input
    // via io::stdin(), but can't use stdin for test cases.

    #[test]
    fn test_single_line_input() {
        let input = b"Hello";
        let expected_result = vec!["Hello".to_string()];
        let actual_result = parse_by_line(&input[..]);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_multi_line_input() {
        let input = b"Hello\nWhat\nIs\nUp";
        let expected_result = vec![
            String::from("Hello"),
            String::from("What"),
            String::from("Is"),
            String::from("Up"),
        ];
        let actual_result = parse_by_line(&input[..]);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_lines_are_trimmed() {
        let input = b"Yo\tDawg\t\nSup   ";
        let expected_result = vec![String::from("Yo\tDawg"), String::from("Sup")];
        let actual_result = parse_by_line(&input[..]);

        assert_eq!(actual_result, expected_result);
    }
}
