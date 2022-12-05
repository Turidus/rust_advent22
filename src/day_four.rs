pub mod puzzle_one {
    use crate::day_four::utils::check_line;

    pub fn run(input: String) -> i32{
        let lines = input.lines();
        let mut answer:i32 = 0;
        for line in lines {
            if check_line(line) {answer += 1}
        }
        return answer
    }

}

pub mod puzzle_two {
    use crate::day_four::utils::check_line_overlap;

    pub fn run(input: String) -> i32{
        let lines = input.lines();
        let mut answer:i32 = 0;
        for line in lines {
            if check_line_overlap(line) {answer += 1}
        }
        return answer
    }
}

mod utils {
    use lazy_regex::{regex_captures};

    pub fn check_line(line: &str) -> bool{
        let result = regex_captures!(r#"(\d+)-(\d+),(\d+)-(\d+)"#, line).unwrap();
        check_if_second_in_first(result) || check_if_first_in_second(result)
    }

    fn check_if_second_in_first(result: (&str,&str,&str,&str,&str)) -> bool{
        check_if_inside(result.1,result.3, result.2, result.4)
    }

    fn check_if_first_in_second(result: (&str,&str,&str,&str,&str)) -> bool{
        check_if_inside(result.3,result.1, result.4, result.2)
    }

    fn check_if_inside(b_lower: &str, i_lower: &str, b_higher: &str, i_higher: &str) -> bool{
        b_lower.parse::<i32>().unwrap() <= i_lower.parse::<i32>().unwrap() && b_higher.parse::<i32>().unwrap() >= i_higher.parse::<i32>().unwrap()
    }

    pub fn check_line_overlap(line: &str) -> bool {
        let result = regex_captures!(r#"(\d+)-(\d+),(\d+)-(\d+)"#, line).unwrap();
        check_if_overlap(result)
    }

    fn check_if_overlap(result: (&str,&str,&str,&str,&str)) -> bool{
        !(result.1.parse::<i32>().unwrap() > result.4.parse::<i32>().unwrap() || result.2.parse::<i32>().unwrap() < result.3.parse::<i32>().unwrap())
    }
}



#[cfg(test)]
mod test_utils {
    use crate::day_four::utils::{check_line, check_line_overlap};

    #[test]
    pub fn correctly_check_line(){
        assert!(check_line("1-6,2-3"));
        assert!(check_line("2-3,1-6"));
        assert!(!check_line("2-3,1-2"));
        assert!(!check_line("2-8,1-6"));
        assert!(!check_line("2-8,7-20"));
    }
    #[test]
    pub fn correctly_check_line_for_overlap() {
        assert!(check_line_overlap("1-6,2-8"));
        assert!(check_line_overlap("2-8,1-6"));
        assert!(!check_line_overlap("2-8,9-12"));
        assert!(!check_line_overlap("9-12,2-8"));
    }
}

#[cfg(test)]
mod tests_p1 {

}

#[cfg(test)]
mod tests_p2 {

}
