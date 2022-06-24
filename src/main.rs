struct Solution {

}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut digits = vec![];

        let mut num = x;

        while num > 0 {
            digits.push(num%10);
            num = num/10;
        }
        let mut val = 0;
        for i in (0..digits.len()).rev() {
            let mut num: i32 = 10;
            num = num.pow(digits.len() as u32 - 1 - i as u32);
            val += digits[i]*num
        }

        return val == x;

        unreachable!()
    }
}

fn main() {
    let ans = Solution::is_palindrome(121);
    println!("{ans}");
}



