use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg: &str = &args[1];
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, 0");

    let mut operator: char = '+';
    let mut num: String = "".to_string();
    let arg = format!("{}{}", arg, "e");
    for char in arg.chars() {
        if char == '+' || char == '-' || char == 'e' {
            if operator == '+' {
                println!("  add rax, {}", num.parse::<usize>().unwrap());
            }
            if operator == '-' {
                println!("  sub rax, {}", num.parse::<usize>().unwrap());
            }
            operator = char;
            num = "".to_string();
        } else {
            num += &char.to_string();
        }
    }

    println!("  ret")
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::process::Command;
    use std::str::from_utf8;

    #[test]
    fn test() {
        run_test("5+21-4", "0\n");
        run_test("4+9-1+7+7", "0\n")
    }

    fn run_test(input: &str, expected: &str) {
        let assembly_ouput = Command::new("cargo")
            .arg("run")
            .arg(input)
            .output()
            .expect("failed to execute process");
        let assembly_str = from_utf8(&assembly_ouput.stdout).unwrap();
        let file = File::create(Path::new("./test.s")).unwrap();
        write!(&file, "{}", assembly_str).unwrap();
        Command::new("gcc")
            .arg("-o")
            .arg("test")
            .arg("test.s")
            .output()
            .expect("failed to execute process");
        Command::new("sh").arg("test");
        let output = Command::new("sh")
            .arg("-c")
            .arg("echo $?")
            .output()
            .expect("failed to execute process");
        let output_str = from_utf8(&output.stdout).unwrap();
        assert_eq!(output_str, expected);
    }
}
