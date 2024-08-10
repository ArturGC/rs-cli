use std::env::Args;
use std::fs::{self, File};
use std::io::Write;

enum FormatFlag {
    B,
    E,
    N,
}

impl FormatFlag {
    fn new(flag: String) -> Self {
        match &flag[..] {
            "-b" => Self::B,
            "-e" => Self::E,
            "-n" => Self::N,
            _ => panic!("Flag not implemented"),
        }
    }

    fn format_b(content: String) -> String {
        {
            let mut result = String::new();
            let mut position = 0;

            for line in content.lines() {
                let new_line = match line.len() {
                    0 => String::from("\n"),
                    _ => {
                        position += 1;

                        format!("{: >5} {}\n", position, line)
                    }
                };

                result.push_str(&new_line);
            }

            result
        }
    }

    fn format_e(file_content: String) -> String {
        file_content
            .lines()
            .fold(String::new(), |acc, line| format!("{}{}$\n", acc, line))
    }

    fn format_n(file_content: String) -> String {
        file_content
            .lines()
            .enumerate()
            .fold(String::new(), |acc, (position, line)| {
                format!("{}{: >5} {}\n", acc, (1 + position), line)
            })
    }
}

enum CatImpl {
    Copy {
        file_origin: String,
        file_destiny: String,
    },
    Concatenate {
        files_origin: Vec<String>,
        file_destiny: String,
    },
    Show {
        file_path: String,
    },
    ShowFormatted {
        file_path: String,
        flag: FormatFlag,
    },
}

impl CatImpl {
    fn new(args: Args) -> CatImpl {
        match args.len() {
            0 => panic!("\"file_name\" argument expected"),
            1 => Self::new_show(args),
            2 => Self::new_show_formatted(args),
            3 => Self::new_copy(args),
            _ => Self::new_concatenate(args),
        }
    }

    fn new_show(mut args: Args) -> Self {
        CatImpl::Show {
            file_path: args.next().unwrap(),
        }
    }

    fn new_show_formatted(mut args: Args) -> Self {
        let flag = FormatFlag::new(args.next().unwrap());
        let file_path = args.next().unwrap();

        CatImpl::ShowFormatted { flag, file_path }
    }

    fn new_copy(mut args: Args) -> Self {
        let file_origin = args.next().unwrap();
        let flag = args.next().unwrap();
        let file_destiny = args.next().unwrap();

        if flag != ">" {
            panic!("Command format expected: cat file_origin > file_destiny");
        }

        CatImpl::Copy {
            file_origin,
            file_destiny,
        }
    }

    fn new_concatenate(mut args: Args) -> Self {
        let mut files_origin: Vec<String> = vec![];

        while args.len() > 2 {
            files_origin.push(args.next().unwrap());
        }

        let flag = args.next().unwrap();

        if flag != ">" {
            panic!("Command format expected: cat file_origin > file_destiny");
        }

        let file_destiny = args.next().unwrap();

        CatImpl::Concatenate {
            files_origin,
            file_destiny,
        }
    }
}

pub fn run(args: Args) {
    let implementation = CatImpl::new(args);

    match implementation {
        CatImpl::Copy {
            file_origin,
            file_destiny,
        } => run_copy(file_origin, file_destiny),
        CatImpl::Concatenate {
            files_origin,
            file_destiny,
        } => run_concatenate(files_origin, file_destiny),
        CatImpl::Show { file_path } => run_show(file_path),
        CatImpl::ShowFormatted { file_path, flag } => run_show_formatted(file_path, flag),
    }
}

fn run_copy(file_origin: String, file_destiny: String) {
    let content_origin = fs::read(file_origin).expect("File not found");
    let mut file = File::create(file_destiny).expect("Error opening the destiny file");

    file.write_all(&content_origin).unwrap();
}

fn run_concatenate(files_origin: Vec<String>, file_destiny: String) {
    let mut file = File::create(file_destiny).expect("Error opening the destiny file");

    for file_origin in files_origin {
        let content_origin = fs::read(file_origin).expect("File not found");

        file.write_all(&content_origin).unwrap();
    }
}

fn run_show(file_path: String) {
    let file_content = fs::read_to_string(file_path).expect("File not found");

    println!("{file_content}");
}

fn run_show_formatted(file_path: String, flag: FormatFlag) {
    let file_content = fs::read_to_string(file_path).expect("File not found");
    let file_content_formatted = match flag {
        FormatFlag::B => FormatFlag::format_b(file_content),
        FormatFlag::E => FormatFlag::format_e(file_content),
        FormatFlag::N => FormatFlag::format_n(file_content),
    };

    println!("{file_content_formatted}");
}
