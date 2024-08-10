use std::env::Args;

enum Flag {
    E,
}

impl Flag {
    fn new(flag: String) -> Self {
        match &flag[..] {
            "-e" => Self::E,
            _ => panic!("Flag not implemented"),
        }
    }

    fn process(string: String) -> String {
        format!("{string}")
    }

    fn process_e(string: String) -> String {
        let mut result = String::new();
        let mut handle_as_scape_key = false;

        for character in string.chars() {
            if character == '\\' {
                handle_as_scape_key = true;
                continue;
            }

            if handle_as_scape_key {
                if character == 'b' && result.ends_with(' ') {
                    result.pop();
                }

                if character == 'n' {
                    result.push_str("\n");
                }

                if character == '\\' {
                    result.push_str("\\");
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

                handle_as_scape_key = false;

                continue;
            }

            result.push(character);
        }

        format!("{result}")
    }
}

enum EchoImpl {
    Show { string: String },
    ShowFormatted { flag: Flag, string: String },
}

impl EchoImpl {
    fn new(args: Args) -> Self {
        match args.len() {
            0 => panic!("\"string\" argument expected"),
            1 => Self::new_show(args),
            2 => Self::new_show_formatted(args),
            _ => panic!("Not implemented"),
        }
    }

    fn new_show(mut args: Args) -> Self {
        Self::Show {
            string: args.next().unwrap(),
        }
    }

    fn new_show_formatted(mut args: Args) -> Self {
        Self::ShowFormatted {
            flag: Flag::new(args.next().unwrap()),
            string: args.next().unwrap(),
        }
    }
}

pub fn run(args: Args) {
    let implementation = EchoImpl::new(args);

    let result = match implementation {
        EchoImpl::Show { string } => Flag::process(string),
        EchoImpl::ShowFormatted { flag, string } => match flag {
            Flag::E => Flag::process_e(string),
        },
    };

    println!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_flag_test() {
        assert_eq!(
            Flag::process(String::from("Geeks for Geeks")),
            "Geeks for Geeks"
        );
    }

    #[test]
    fn e_flag_test_b() {
        assert_eq!(
            Flag::process_e(String::from("Geeks \\bfor \\bGeeks")),
            "GeeksforGeeks"
        );
    }

    #[test]
    fn e_flag_test_c() {
        assert_eq!(
            Flag::process_e(String::from("Geeks \\cfor Geeks")),
            "Geeks "
        );
    }

    #[test]
    fn e_flag_test_n() {
        assert_eq!(
            Flag::process_e(String::from("Geeks \\nfor \\nGeeks")),
            "Geeks \nfor \nGeeks"
        );
    }
}
