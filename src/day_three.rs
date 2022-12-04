pub mod puzzle_one {
    use crate::day_three::utils::{find_double, get_vector_from_string, split_vectors};
    pub fn run(input: String) -> i32{
        let lines = input.lines();
        let mut answer:i32 = 0;
        for line in lines {
            answer += find_double(split_vectors(get_vector_from_string(line)))
        }
        return answer
    }

}

pub mod puzzle_two {
    use crate::day_three::utils::{find_triple, get_vector_from_string, sort_vectors_into_triples};

    pub fn run(input: String) -> i32{
        let lines = input.lines();
        let mut answer = 0;
        let mut input_vec: Vec<Vec<i32>> = Vec::new();
        for line in lines {
            input_vec.push(get_vector_from_string(line))
        }
        let triple_vec = sort_vectors_into_triples(input_vec);
        for vec in triple_vec {
            answer += find_triple(&vec);
        }
        return answer;
    }
}

mod utils {
    pub fn get_number_from_letter(input: &str) -> i32 {
        match input {
            "a" => 1,
            "b" => 2,
            "c" => 3,
            "d" => 4,
            "e" => 5,
            "f" => 6,
            "g" => 7,
            "h" => 8,
            "i" => 9,
            "j" => 10,
            "k" => 11,
            "l" => 12,
            "m" => 13,
            "n" => 14,
            "o" => 15,
            "p" => 16,
            "q" => 17,
            "r" => 18,
            "s" => 19,
            "t" => 20,
            "u" => 21,
            "v" => 22,
            "w" => 23,
            "x" => 24,
            "y" => 25,
            "z" => 26,
            "A" => 27,
            "B" => 28,
            "C" => 29,
            "D" => 30,
            "E" => 31,
            "F" => 32,
            "G" => 33,
            "H" => 34,
            "I" => 35,
            "J" => 36,
            "K" => 37,
            "L" => 38,
            "M" => 39,
            "N" => 40,
            "O" => 41,
            "P" => 42,
            "Q" => 43,
            "R" => 44,
            "S" => 45,
            "T" => 46,
            "U" => 47,
            "V" => 48,
            "W" => 49,
            "X" => 50,
            "Y" => 51,
            "Z" => 52,
            _ => panic!("Unknown Letter")
        }
    }

    pub fn get_vector_from_string(input: &str) -> Vec<i32>{
        let split = input.chars().map(|c| get_number_from_letter(&*c.to_string())).collect();
        return split
    }

    pub fn split_vectors(vec: Vec<i32>) -> Vec<Vec<i32>> {
        vec.chunks(vec.len() / 2).map(|s| s.into()).collect()
    }

    pub fn find_double(vec_halfs: Vec<Vec<i32>>) -> i32 {
        let mut double = 0;
        for i in &vec_halfs[0] {
            if vec_halfs[1].contains(i) {
                double = *i;
                break
            }
        }
        if double == 0 {panic!("No double found")}
        return double;
    }

    pub fn sort_vectors_into_triples(vecs: Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>>{
        let mut triple_vecs: Vec<Vec<Vec<i32>>> = Vec::new();
        let mut single_triple: Vec<Vec<i32>> = Vec::new();
        for vec in vecs {
            single_triple.push(vec);
            if single_triple.len() == 3 {
                triple_vecs.push(single_triple);
                single_triple = Vec::new()
            }
        }
        return triple_vecs;
    }

    pub fn find_triple(vecs: &Vec<Vec<i32>>) -> i32 {
        if vecs.len() != 3 {panic!("Wrong vector length.")};
        for i in &vecs[0] {
            if vecs[1].contains(&i) && vecs[2].contains(&i) {return *i}
        }
        panic!("Was not able to find a triple!")
    }
}



#[cfg(test)]
mod test_utils {
    use crate::day_three::utils::*;

    #[test]
    fn right_number_from_letter() {
        assert_eq!(1,get_number_from_letter("a"));
        assert_eq!(2,get_number_from_letter("b"));
        assert_eq!(26,get_number_from_letter("z"));
        assert_eq!(52,get_number_from_letter("Z"))
    }

    #[test]
    fn transform_input_vectors_with_ints(){
        assert_eq!(vec![1,2,3,52,51,1], get_vector_from_string("abcZYa"))
    }

    #[test]
    fn split_vector_nicely(){
        let vec_halfs = split_vectors(get_vector_from_string("abcZYa"));
        assert_eq!(vec![1,2,3], vec_halfs[0]);
        assert_eq!(vec![52,51,1], vec_halfs[1]);
    }

    #[test]
    fn find_correct_double(){
        let vec_halfs = split_vectors(get_vector_from_string("abcZYa"));
        assert_eq!(1, find_double(vec_halfs))
    }

    #[test]
    fn sort_vectors_into_correct_triples(){
        let mut vecs: Vec<Vec<i32>> = Vec::new();
        vecs.push(vec![1]);
        vecs.push(vec![2]);
        vecs.push(vec![3]);
        vecs.push(vec![4]);
        vecs.push(vec![5]);
        vecs.push(vec![6]);
        let triple_vecs = sort_vectors_into_triples(vecs);
        assert_eq!(3, triple_vecs[0].len());
        assert_eq!(3, triple_vecs[1].len());
    }

    #[test]
    fn find_correct_triple(){
        let mut vecs: Vec<Vec<i32>> = Vec::new();
        vecs.push(vec![1,2,3]);
        vecs.push(vec![1,4,5]);
        vecs.push(vec![6,7,1]);
        assert_eq!(1, find_triple(&vecs));
    }
}

#[cfg(test)]
mod tests_p1 {
    use crate::day_three::puzzle_one::*;
    #[test]
    fn it_works(){
        assert_eq!(2,run("abcZYa\nabcZYa".to_string()))

    }
}

#[cfg(test)]
mod tests_p2 {
    use crate::day_three::puzzle_two::*;

    #[test]
    fn it_works(){
        assert_eq!(3,run("abc\nade\nafg\nbac\nbed\nghb".to_string()))
    }
}
