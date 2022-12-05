pub mod puzzle_one {
    use crate::day_five::utils::{Commands, parse_command_line, parse_stack_line, Stacks};

    pub fn run(input: String) -> i32{
        let lines = input.lines();
        let mut stacks:Stacks = Stacks::new();
        let mut commands:Commands = Commands::new();
        for line in lines {
            if line.contains("[") {parse_stack_line(&mut stacks, line)}
            else if line.contains("move") { parse_command_line(&mut commands, line)}
        }
        for tuple in commands.command_list { stacks.move_values(tuple.0, tuple.1, tuple.2) }
        println!("{}", stacks.get_result());
        return 0
    }

}

pub mod puzzle_two {
    use crate::day_five::utils::{Commands, parse_command_line, parse_stack_line, Stacks};

    pub fn run(input: String) -> i32{
        let lines = input.lines();
        let mut stacks:Stacks = Stacks::new();
        let mut commands:Commands = Commands::new();
        for line in lines {
            if line.contains("[") {parse_stack_line(&mut stacks, line)}
            else if line.contains("move") { parse_command_line(&mut commands, line)}
        }
        for tuple in commands.command_list { stacks.move_values_at_once(tuple.0, tuple.1, tuple.2) }
        println!("{}", stacks.get_result());
        return 0
    }

}

mod utils {
    use lazy_regex::{regex_captures};
    pub fn pad_string(line: &str) -> String {
        format!("{: <35}", line)
    }

    pub fn parse_stack_line(stacks: &mut Stacks, line: &str) {
        let padded_string = pad_string(line);
        let result = regex_captures!(r#"(\s{3}|\[\w\])\s(\s{3}|\[\w\])\s(\s{3}|\[\w\])\s(\s{3}|\[\w\])\s(\s{3}|\[\w\])\s(\s{3}|\[\w\])\s(\s{3}|\[\w\])\s(\s{3}|\[\w\])\s(\s{3}|\[\w\])"#, padded_string.as_str()).unwrap();
        let result_vec = get_result_vector(result);
        let mut index: usize = 1;
        for string in result_vec {
            if string.contains("[") {
                let letter = parse_letter_from_string(string);
                stacks.stacks[index].push(letter);
            };
            index += 1;
        }
    }

    pub fn parse_command_line(commands: &mut Commands, line: &str){
        let result = regex_captures!(r#"move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)"#,line).unwrap();
        commands.add_command(result.1.parse().unwrap(), result.2.parse().unwrap(), result.3.parse().unwrap(),);
    }

    fn get_result_vector(result: (&str, &str, &str, &str, &str, &str, &str, &str, &str, &str)) -> Vec<String>{
        let mut result_vec: Vec<String> = Vec::new();
        result_vec.push(result.1.parse().unwrap());
        result_vec.push(result.2.parse().unwrap());
        result_vec.push(result.3.parse().unwrap());
        result_vec.push(result.4.parse().unwrap());
        result_vec.push(result.5.parse().unwrap());
        result_vec.push(result.6.parse().unwrap());
        result_vec.push(result.7.parse().unwrap());
        result_vec.push(result.8.parse().unwrap());
        result_vec.push(result.9.parse().unwrap());
        return result_vec
    }

    fn parse_letter_from_string(string: String) -> String {
        string[1..2].parse().unwrap()
    }

    pub struct Stacks {
        pub stacks: Vec<Vec<String>>
    }
    impl Stacks {
        pub fn new() -> Stacks{
            let mut vec: Vec<Vec<String>> = Vec::new();
            for _i in 0..10 {vec.push(Vec::new())}
            Stacks {
                stacks: vec
            }
        }
        pub fn move_values(&mut self, amount: i32, from: usize, to: usize){
            for _i in 0..amount {
                let value = self.stacks[from].remove(0);
                self.stacks[to].insert(0, value);
            }
        }
        pub fn move_values_at_once(&mut self, amount: i32, from: usize, to: usize){
            let mut temp_vec: Vec<String> = Vec::new();
            for _i in 0..amount {
                let value = self.stacks[from].remove(0);
                temp_vec.insert(0,value)
            }
            for value in temp_vec {
                self.stacks[to].insert(0, value)
            }
        }
        pub fn get_result(&self) -> String {
            let mut string: String = "".parse().unwrap();
            for i in 1..10 {
                if self.stacks[i].len() > 0 {
                    string += &*self.stacks[i][0]
                }
            }
            return string
        }
    }

    pub struct Commands {
        pub command_list: Vec<(i32,usize,usize)>
    }
    impl Commands {
        pub fn new() -> Commands {
            Commands {
                command_list: Vec::new()
            }
        }
        pub fn add_command(&mut self, ammount: i32, from: usize, to: usize){
            self.command_list.push((ammount, from, to));
        }
    }
}



#[cfg(test)]
mod test_utils {
    use crate::day_five::utils::{Commands, pad_string, parse_command_line, parse_stack_line, Stacks};

    #[test]
    fn correctly_pad_string() {
        assert_eq!("        [F] [Q]         [Q]        ", pad_string("        [F] [Q]         [Q]"));
        assert_eq!("[V] [M] [B] [G] [S] [C] [T] [V] [S]", pad_string("[V] [M] [B] [G] [S] [C] [T] [V] [S]"))
    }

    #[test]
    fn stack_is_useful(){
        let mut stacks = Stacks::new();
        stacks.stacks[2].push("Test".to_string());
        stacks.stacks[2].push("Test2".to_string());
        stacks.stacks[2].push("Test3".to_string());
        assert_eq!("Test".to_string(),stacks.stacks[2][0]);
        stacks.move_values(1,2,3);
        assert_eq!("Test".to_string(),stacks.stacks[3][0]);
        stacks.move_values(2,2,3);
        assert_eq!("Test3".to_string(),stacks.stacks[3][0]);
        assert_eq!("Test2".to_string(),stacks.stacks[3][1]);
        assert_eq!("Test".to_string(),stacks.stacks[3][2]);
        assert_eq!("Test3", stacks.get_result())

    }

    #[test]
    fn parsing_line_correctly(){
        let mut stacks = Stacks::new();
        parse_stack_line(&mut stacks, "[V] [M] [B] [G] [S] [C] [T] [V] [S]");
        assert_eq!("V", stacks.stacks[1][0]);
        assert_eq!("M", stacks.stacks[2][0]);
        assert_eq!("B", stacks.stacks[3][0]);
        assert_eq!("G", stacks.stacks[4][0]);
        assert_eq!("S", stacks.stacks[5][0]);
        assert_eq!("C", stacks.stacks[6][0]);
        assert_eq!("T", stacks.stacks[7][0]);
        assert_eq!("V", stacks.stacks[8][0]);
        assert_eq!("S", stacks.stacks[9][0]);
        parse_stack_line(&mut stacks, "        [F] [Q]         [Q]");
        assert_eq!("V", stacks.stacks[1][0]);
        assert_eq!("F", stacks.stacks[3][1]);
    }

    #[test]
    fn commands_are_useful(){
        let mut commands = Commands::new();
        commands.add_command(1, 2, 3);
        assert_eq!((1,2,3), commands.command_list[0])
    }

    #[test]
    fn parse_command_line_correctly(){
        let mut commands = Commands::new();
        parse_command_line(&mut commands, "move 14 from 3 to 9");
        assert_eq!((14,3,9), commands.command_list[0])
    }

}

#[cfg(test)]
mod tests_p1 {
    use crate::day_five::puzzle_one::run;

    #[test]
    fn run_test(){
        assert_eq!(0, run("[J] [V] [W] [M] [F]     [J]     [J]\n[Z] [G] [S] [W] [N] [D] [R]     [T]\nmove 1 from 1 to 2".parse().unwrap()))
    }
}

#[cfg(test)]
mod tests_p2 {
    use crate::day_five::puzzle_two::run;

    #[test]
    fn run_test(){
        assert_eq!(0, run("[J] [V] [W] [M] [F]     [J]     [J]\n[Z] [G] [S] [W] [N] [D] [R]     [T]\nmove 1 from 1 to 2".parse().unwrap()))
    }
}
