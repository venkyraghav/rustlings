// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!

// TODO I AM NOT DONE

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(&str, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];

        for (string, command) in input.iter() {
            match command {
                Command::Uppercase => output.push(string.clone().to_uppercase()),
                Command::Trim => output.push(string.trim().to_string()),
                Command::Append(size) => {
                    let s1 = "bar".repeat(*size);
                    let s2 = string.to_string() + s1.as_str();
                    output.push(s2);
                }
            }
        }
        output
    }

    /*
    pub fn transformer_todo_returnstr(input: Vec<(&str, Command)>) -> Vec<&str> {
        let mut output: Vec<&str> = vec![];

        for (string, command) in input.iter() {
            match command {
                Command::Uppercase => {
                    let s1 = string.to_uppercase();
                    output.push(&s1);
                }
                Command::Trim => output.push(string.trim()),
                Command::Append(size) => {
                    let s1 = "bar".repeat(*size);
                    let mut s2 = string.to_string();
                    s2.push_str(s1.as_str());
                    output.push(*string);
                }
            }
        }
        output
    }
     */
}

#[cfg(test)]
mod tests {
    use super::Command;
    use my_module::transformer;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
