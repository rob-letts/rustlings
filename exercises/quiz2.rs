pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];

        for (string, command) in input.iter() {
            let res = match command {
              Command::Uppercase => string.to_uppercase(),
              Command::Trim => String::from(string.trim()),
              Command::Append(num) => {
                let mut res = String::from(string);
                res.push_str("bar".repeat(*num).as_str());
                res
              },
            };

            output.push(res);
          }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        let res = assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
