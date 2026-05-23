pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub mod transformer {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = vec![];
        for each in input {
            match each.1 {
                Command::Uppercase => output.push(each.0.to_uppercase()),
                Command::Trim => output.push(each.0.trim().to_string()),
                Command::Append(num) => {
                    let mut new_string = String::from(&each.0);
                    for _ in 0..num {
                        new_string.push_str("bar");
                    }
                    
                    output.push(new_string);
                }
            }
        }
        output
    }
}