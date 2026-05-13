use std::collections::{HashMap, HashSet, LinkedList};

struct Solution {

}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for s in strs.into_iter() {
            let str = s.as_str();
            let chars = str.chars();
            let mut pane = [0u8;10000];
            for x in chars {
                let x = x.to_ascii_lowercase() as u8;
                let pos = x - 97;
                pane[pos as usize] += 1;
            }
            map.entry(pane).or_insert_with(Vec::new).push(s);
        }
        map.into_iter().map(|(k,v)| v).collect()
    }

    pub fn union_find(nums_ref: &mut [u32],x: u32) -> u32 {
        if nums_ref[x as usize] == x {return x}
        nums_ref[x as usize] = Self::union_find(nums_ref, nums_ref[x as usize]);
        nums_ref[x as usize]
    }

    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {return 0;}
        let set = nums.into_iter().collect::<HashSet<_>>();
        let mut max = 0;
        for num in set.iter() {
            if set.contains(&(*num - 1)) { continue; }
            let mut shift = *num ;
            while set.contains(&shift) {
                shift += 1;
            }
            let len = shift - *num;
            if len > max { max = len }
        }
        max
    }
    // 利用快排
    pub fn longest_consecutive2(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {return 0;}
        let mut nums = nums;
        nums.sort_unstable();
        let mut curlen = 1;
        let mut maxlen = 1;
        let len = nums.len();
        for i in 1..len {
            if nums[i] == nums[i-1] + 1{
                curlen += 1;
            }else if nums[i] == nums[i-1]{
                continue
            }else {
                curlen = 1;continue;
            }
            if maxlen < curlen {maxlen = curlen;}
        }
        maxlen
    }

    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() == 1 {return;}
        let mut posvec = Vec::new();
        let mut pos = 0;
        let len = nums.len();
        for i in nums.iter(){
            if *i == 0 {
                posvec.push(pos);
            }
            pos += 1;
        }
        let count = posvec.len();
        let mut shift = 0;
        for i in posvec{
            for j in i-shift..len-shift-1{
                nums[j] = nums[j+1]
            }
            shift+=1;
        }
        for i in len-count..len{
            nums[i] = 0;
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::{Solution};
    #[test]
    fn test_move_zeroes() {
        // let mut nums = vec![0,1,0,3,12];
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        println!("{:?}", nums);
    }


    #[test]
    fn test_group_anagrams() {
        let list = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
        let list = Solution::group_anagrams(list);

    }

    #[test]
    fn test_longest_consecutive() {
        // let num = vec![100,4,200,1,3,2];
        // let num = vec![-6,-9,8,-8,-1,-3,-6,8,-9,-1,-4,-8,-5,0,1,6,-8,-5,-7,8,-2,-8,4,5,-5,-1,-5];
        // let num = vec![-1,9,-3,-6,7,-8,-6,2,9,2,3,-2,4,-1,0,6,1,-9,6,8,6,5,2];
        let num = vec![-6,6,-3,3,0,8,4,-6,-4,2,-8,-1,-2,9,-3,-9,2,-9,-2,8,5,-7,9,-7,7,-8,5,8];
        // let num = vec!;
        // let num = vec![1,0,1,2];
        // let num = vec![0,3,7,2,5,8,4,6,0,1];
        let max = Solution::longest_consecutive(num);
        println!("max = {}", max);
    }


    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }




}


// pub fn longest_consecutive_abort(nums: Vec<i32>) -> i32 {
//     let nums = nums.into_iter().collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
//     println!("after ");
//     println!("{:?}", nums);
//     let len = nums.len();
//     let mut pos_vec = [0u32;10000];
//     for i in 0..len {
//         pos_vec[i] = i as u32;
//     }
//     for i in 0..len {
//         let consentrate = nums[i];
//         pos_vec[i]= pos_vec[pos_vec[i] as usize];
//         let minus1 = consentrate - 1;
//         let add1 = consentrate + 1;
//         for j in i+1..len {
//             if nums[j] == minus1 || nums[j] == add1 {
//                 //默认皈依左边，如果外循环指针指向自己，内循环（右边指针）：
//                 //      指向自己，无任何外部关系： 使其指向自己
//                 //      指向别人：指向别人的父节点
//                 //如果外循环指针指向别人，内循环：
//                 //      指向自己： 赋值外循环父节点
//                 //      指向别人： 递归赋值外循环父节点
//                 if pos_vec[i] == i as u32 && pos_vec[j] == j as u32 {
//                     // println!("both equal ");
//                     pos_vec[j] = i as u32;
//                     // println!("debug : {},{}", j,pos_vec[j]);
//
//                 }
//                 else if pos_vec[i] != i as u32 && pos_vec[j] == j as u32 {
//                     // println!("lt not equal");
//                     pos_vec[j] = pos_vec[i];
//                     // println!("debug : {},{}", j,pos_vec[j]);
//
//                 }
//                 else if pos_vec[i] == i as u32 && pos_vec[j] != j as u32{
//                     // println!("rt not equal");
//                     pos_vec[i] = pos_vec[j];
//                     // println!("debug : {},{}", i,pos_vec[i]);
//                 }
//                 else{
//                     // println!("both not equal ");
//                     if pos_vec[i] <= pos_vec[j] { pos_vec[pos_vec[j] as usize] = pos_vec[i]; pos_vec[j] = pos_vec[i]; }
//                     //低级错误留念
//                     // else { pos_vec[i] = pos_vec[j]; pos_vec[pos_vec[i] as usize] = pos_vec[j];}
//                     else { pos_vec[pos_vec[i] as usize] = pos_vec[j];pos_vec[i] = pos_vec[j]; }
//
//                     // println!("debug : {},{}", j,pos_vec[j]);
//                 }
//                 continue
//             }
//         }
//
//         // println!("{}. {:?}\n", i+1,pos_vec);
//     }
//     for i in 0..len{
//         Self::union_find(&mut pos_vec,i as u32);
//     }
//     let mut map:HashMap<_,u32> = HashMap::new();
//     for i in 0..len {
//         let value = map.entry(pos_vec[i]).or_insert_with(|| 0);
//         *value += 1;
//     }
//     let mut max = 0 ;
//     for (_,v) in map.into_iter(){
//         if v > max {max = v;}
//     }
//     max as i32
// }