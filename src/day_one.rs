pub mod puzzle_one {
    use std::cmp::max;


    pub fn run(input: String) -> i32{
        let lines = input.lines();
        let mut answer:i32 = 0;
        let mut acc:i32 = 0;
        for line in lines{
            if line.is_empty() {
                answer = max(answer,acc);
                acc = 0
            }
            else {
                acc += line.parse::<i32>().expect("String was not a number");
            }
        }
        answer = max(answer,acc);
        return answer
    }
}

pub mod puzzle_two {
    pub fn run(input: String) -> i32{
        let lines = input.lines();
        let mut answer_list:Vec<i32> = Vec::new();
        let mut acc:i32 = 0;
        for line in lines{
            if line.is_empty() {
                answer_list.push(acc);
                acc = 0
            }
            else {
                acc += line.parse::<i32>().expect("String was not a number");
            }
        }
        answer_list.push(acc);
        answer_list.sort_by(|a, b| b.cmp(a));
        return answer_list[0] + answer_list[1] + answer_list[2]
    }
}

#[cfg(test)]
mod tests_p1 {
    use crate::day_one::puzzle_one::*;

    #[test]
    fn it_works(){
        assert_eq!(11, run("2\n3\n4\n\n5\n6".parse().unwrap()))
    }
}

#[cfg(test)]
mod tests_p2 {
    use crate::day_one::puzzle_two::*;

    #[test]
    fn it_works(){
        assert_eq!(9, run("1\n\n2\n\n3\n\n4".parse().unwrap()))
    }
}
