pub mod puzzle_one {
    use std::cmp::max;
    use crate::day_two::utils::{find_round_result, get_number_from_letter};


    pub fn run(input: String) -> i32{
        let lines = input.lines();
        let mut answer:i32 = 0;
        for line in lines {
            let split: Vec<&str>= line.split(" ").collect();
            answer += get_number_from_letter(split[1]) + find_round_result(split[0], split[1])
        }
        return answer
    }
}

pub mod puzzle_two {
    use crate::day_two::utils::{find_correct_round_match, get_result_from_letter};

    pub fn run(input: String) -> i32{
        let lines = input.lines();
        let mut answer:i32 = 0;
        for line in lines {
            let split: Vec<&str>= line.split(" ").collect();
            answer += get_result_from_letter(split[1]) + find_correct_round_match(split[0], split[1])
        }

        return answer
    }
}

mod utils {
    use crate::day_two::puzzle_two::run;

    pub fn get_number_from_letter(input: &str) -> i32 {
        match input {
            "A"|"X" => return 1,
            "B"|"Y" => return 2,
            "C"|"Z" => return 3,
            _ => panic!("Unknown Letter")
        }
    }

    pub fn find_round_result(player1: &str, player2: &str) -> i32 {
        let p1 = get_number_from_letter(player1);
        let p2 = get_number_from_letter(player2);
        return if p1 == p2 { 3 } else if p1 == 1 && p2 == 3 || p1 == 2 && p2 == 1 || p1 == 3 && p2 == 2 { 0 } else { 6 }
    }

    pub fn get_result_from_letter(input: &str) -> i32 {
        match input {
            "X" => return 0,
            "Y" => return 3,
            "Z" => return 6,
            _ => panic!("Unknown Letter")
        }
    }

    pub fn find_correct_round_match(player1: &str, player2: &str) -> i32 {
        let p1 = get_number_from_letter(player1);
        let p2 = get_result_from_letter(player2);
        if p2 == 3 {return p1}
        else if p2 == 0 {
            return if p1 == 1 { 3 } else if p1 == 2 { 1 } else { 2 }
        }
        else {
            return if p1 == 1 { 2 } else if p1 == 2 { 3 } else { 1 }
        }
    }
}



#[cfg(test)]
mod test_utils {
    use crate::day_two::utils::*;

    #[test]
    fn right_number_from_letter() {
        assert_eq!(1, get_number_from_letter("A"));
        assert_eq!(1, get_number_from_letter("X"));
        assert_eq!(2, get_number_from_letter("B"));
        assert_eq!(2, get_number_from_letter("Y"));
        assert_eq!(3, get_number_from_letter("C"));
        assert_eq!(3, get_number_from_letter("Z"));
    }

    #[test]
    fn right_match_results() {
        assert_eq!(0, find_round_result("A", "Z"));
        assert_eq!(0, find_round_result("B", "X"));
        assert_eq!(0, find_round_result("C", "Y"));
        assert_eq!(3, find_round_result("A", "X"));
        assert_eq!(3, find_round_result("B", "Y"));
        assert_eq!(3, find_round_result("C", "Z"));
        assert_eq!(6, find_round_result("A", "Y"));
        assert_eq!(6, find_round_result("B", "Z"));
        assert_eq!(6, find_round_result("C", "X"));
    }

    #[test]
    fn right_result_from_letter(){
        assert_eq!(0, get_result_from_letter("X"));
        assert_eq!(3, get_result_from_letter("Y"));
        assert_eq!(6, get_result_from_letter("Z"));
    }

    #[test]
    fn right_match_choice() {
        assert_eq!(3, find_correct_round_match("A", "X"));
        assert_eq!(1, find_correct_round_match("B", "X"));
        assert_eq!(2, find_correct_round_match("C", "X"));
        assert_eq!(1, find_correct_round_match("A", "Y"));
        assert_eq!(2, find_correct_round_match("B", "Y"));
        assert_eq!(3, find_correct_round_match("C", "Y"));
        assert_eq!(2, find_correct_round_match("A", "Z"));
        assert_eq!(3, find_correct_round_match("B", "Z"));
        assert_eq!(1, find_correct_round_match("C", "Z"));
    }
}

#[cfg(test)]
mod tests_p1 {
    use crate::day_two::puzzle_one::*;
    #[test]
    fn it_works(){
        assert_eq!(9,run("B Z".parse().expect("What?")))
    }
}

#[cfg(test)]
mod tests_p2 {
    use crate::day_two::puzzle_two::*;

    #[test]
    fn it_works(){
        assert_eq!(9,run("B Z".parse().expect("What?")));
        assert_eq!(7,run("C Z".parse().expect("What?")));
    }
}
