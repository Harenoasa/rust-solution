use std::collections::{HashMap, LinkedList};

struct Solution {

}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for s in strs.into_iter() {
            let str = s.as_str();
            let chars = str.chars();
            let mut pane = [0u8;26];
            for x in chars {
                let x = x.to_ascii_lowercase() as u8;
                let pos = x - 97;
                pane[pos as usize] += 1;
            }
            map.entry(pane).or_insert_with(Vec::new).push(s);
        }
        map.into_iter().map(|(k,v)| v).collect()
    }

    pub fn union_find(nums_ref: &mut [u32],) {

    }
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let vec = [0u32;100000];
        for i in 0..len {
            let consentrate = nums[i];
            let minus1 = consentrate - 1;
            let add1 = consentrate + 1;
            for j in i+1..len {
                if nums[j] == minus1 || nums[j] == add1 {
                    Solution::union_find(&mut nums,);
                    break;
                }
            }
        }


        0
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_group_anagrams() {
        let list = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
        let list = Solution::group_anagrams(list);

    }


    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }




}