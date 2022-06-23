use std::collections::HashMap;

struct Solution {

}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<&i32, Vec<i32>> = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            let indicis = map.entry(num).or_insert(vec![]);
            indicis.push(index as i32);
        }

        let mut ans = vec![];

        for (i, num) in nums.iter().enumerate() {
            let num_to_find: i32 = target - num;
            let indicis = match map.get(&num_to_find) {
                Some(arr) => arr.to_owned(),
                None => vec![]
            };
            if indicis.len() == 0 {
                continue;
            }

            else {
                for index in indicis {
                    if index as i32 != i as i32 {
                        ans.push(index as i32);
                        ans.push(i as i32);
                        break;
                    } else {
                        continue;
                    }
                }
            }
            if ans.len() > 0 {
                break;
            }
        }

        return ans;
    }
}

fn main() {
    let ans = Solution::two_sum(vec![2,7,11,15,2], 9);
    println!("{:?}", ans);
}



