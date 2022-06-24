struct Solution {

}



impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        
        for chr in s.chars() {
            if stack.len() > 0 && Self::is_closing(&chr) && Self::get_opening(&chr) == stack[stack.len()-1]  {
                stack.pop();
            } else {
                stack.push(chr);
            }
        }

        return stack.len() == 0;
        
        unreachable!();
    }

    pub fn is_closing(chr: &char) -> bool {
        return chr == &')' || chr == &'}' || chr == &']';
    }

    pub fn get_opening(chr: &char) -> char {
        if chr == &')' { return '(';}
        else if chr == &'}' {return '{';}
        else {return '[';}
    }
}

fn main() {
    let ans = Solution::is_valid("()((])".to_string());
    println!("{ans}");
}



