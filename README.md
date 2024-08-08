# [Rust CLI utilities](https://zerotomastery.io/blog/rust-practice-projects/#Beginner-Rust-Projects)

Command-line utilities are the gateway to learning any programming language. They have simplified input and output systems which makes the code less complex to write. This allows you to focus on the problem at hand and learn Rust instead of learning a specific problem domain.

Rewriting existing tools in Rust can provide a great baseline to get started:

- [`echo`](https://www.geeksforgeeks.org/echo-command-in-linux-with-examples/): repeats input
- [`cat`](https://www.geeksforgeeks.org/cat-command-in-linux-with-examples/#basic-syntax-of-cat-command): concatenates files
- [`ls`](https://www.geeksforgeeks.org/ls-command-in-linux/): list directories
- [`find`](https://www.geeksforgeeks.org/find-command-in-linux-with-examples/): locate files or directgories
- [`grep`](https://www.geeksforgeeks.org/grep-command-in-unixlinux/): matches text in files

### Level up your skills and project by adding in these extra features:

Add extra flags to the tools to enhance their feature set
Use proper error handling so the programs don't end prematurely (no .unwrap() and no .expect())
Add multi-threading support. find and grep are good candidates for this
Add colorized output

### some foundation crates

The Rust standard library is light on features, so here are some foundation crates that are great to use in nearly any project:

- [thiserror](https://crates.io/crates/thiserror): Make custom error types
- [color-eyre](https://crates.io/crates/color-eyre): Report application errors
- [clap](https://crates.io/crates/clap): Process command line arguments
- [serde](https://crates.io/crates/serde): Serialize and de-serialize data structures
