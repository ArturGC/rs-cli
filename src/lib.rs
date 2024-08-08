use std::env::Args;

pub mod echo {
    use super::*;

    pub fn handler(mut args: Args) -> String {
        if args.len() == 0 {
            panic!("\"string\" argument expected");
        }

        if args.len() == 1 {
            let string = args.next().unwrap();

            return process_no_flag(string);
        }

        if args.len() == 2 {
            let option = args.next().unwrap();
            let string = args.next().unwrap();

            return match &option[..] {
                "-e" => process_e_flag(string),
                _ => panic!("Option not implemented"),
            };
        }

        panic!("Not implemented")
    }

    fn process_no_flag(string: String) -> String {
        format!("{string}")
    }

    fn process_e_flag(string: String) -> String {
        let mut result = String::new();
        let mut scape = false;

        for character in string.chars() {
            if character == '\\' {
                scape = true;
                continue;
            }

            if scape {
                if character == 'b' && result.ends_with(' ') {
                    result.pop();
                }

                if character == 'n' {
                    result.push_str("\n");
                }

                if character == 't' {
                    result.push_str("\t");
                }

                if character == 'v' {
                    let spaces = match result.split("\n").last() {
                        Some(line) => format!("\n{}", " ".repeat(line.len())),
                        None => String::new(),
                    };

                    result.push_str(&spaces);
                }

                if character == 'r' {
                    result.clear();
                }

                if character == 'c' {
                    return format!("{result}");
                }

                scape = false;
                continue;
            }

            result.push(character);
        }

        format!("{result}")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn no_flag_test() {
            assert_eq!(
                process_no_flag(String::from("Geeks for Geeks")),
                "Geeks for Geeks"
            );
        }

        #[test]
        fn e_flag_test_b() {
            assert_eq!(
                process_e_flag(String::from("Geeks \\bfor \\bGeeks")),
                "GeeksforGeeks"
            );
        }

        #[test]
        fn e_flag_test_c() {
            assert_eq!(process_e_flag(String::from("Geeks \\cfor Geeks")), "Geeks ");
        }

        #[test]
        fn e_flag_test_n() {
            assert_eq!(
                process_e_flag(String::from("Geeks \\nfor \\nGeeks")),
                "Geeks \nfor \nGeeks"
            );
        }
    }
}
