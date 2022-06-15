struct Solution {

}

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        words.sort_by(|a, b| a.len().cmp(&b.len()));
        println!("{:?}", words);
    
        let mut lens: Vec<i32> = Vec::new();
    
        let mut i = 0;
    
        while i < words.len() {
            let current_word = words[i].as_str();
            let mut longest_predecessor_length = 1;
            let mut j = match i > 0 {
                true => i-1,
                false => 0
            };
            
            loop {
                let prev_word = words[j].as_str();
                
                if Self::is_predecessor(prev_word, current_word) {
                    longest_predecessor_length = match lens[j] + 1 >= longest_predecessor_length {
                        true => lens[j] + 1,
                        false => longest_predecessor_length
                    };
                }
                match j > 0 {
                    true => j -= 1,
                    false => break
                }
            }
            lens.push(longest_predecessor_length);
            i += 1;
        }
    
        lens.iter().max().unwrap().to_owned()
    }
    
    pub fn is_predecessor( word_one: &str, word_two: &str) -> bool {
    
        if word_one.len() + 1 != word_two.len() {
            return false
        }
    
        let ( mut i_one, mut i_two) = (0, 0);
    
        while i_two < word_two.len() {
            match word_one.chars().nth(i_one) {
                Some(c) => {
                    match word_two.chars().nth(i_two) {
                        Some(ct) => {
                            if c == ct {
                                i_one += 1;
                            }
                        },
    
                        None => {}
                    };
                },
                None => {}
            };
            i_two += 1;
        }
    
        i_one == word_one.len()
    }
}


fn main() {
    let mut words = vec!["xbc".to_owned(),"pcxbcf".to_owned(),"xb".to_owned(),"cxbc".to_owned(),"pcxbc".to_owned()];
    let ans = Solution::longest_str_chain(words);
    println!("{ans}");
}



