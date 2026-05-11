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

    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        
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