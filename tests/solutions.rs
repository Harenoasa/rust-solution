use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList};
use std::marker::Destruct;

struct Solution {}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for s in strs.into_iter() {
            let str = s.as_str();
            let chars = str.chars();
            let mut pane = [0u8; 10000];
            for x in chars {
                let x = x.to_ascii_lowercase() as u8;
                let pos = x - 97;
                pane[pos as usize] += 1;
            }
            map.entry(pane).or_insert_with(Vec::new).push(s);
        }
        map.into_iter().map(|(k, v)| v).collect()
    }

    pub fn union_find(nums_ref: &mut [u32], x: u32) -> u32 {
        if nums_ref[x as usize] == x {
            return x;
        }
        nums_ref[x as usize] = Self::union_find(nums_ref, nums_ref[x as usize]);
        nums_ref[x as usize]
    }

    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let set = nums.into_iter().collect::<HashSet<_>>();
        let mut max = 0;
        for num in set.iter() {
            if set.contains(&(*num - 1)) {
                continue;
            }
            let mut shift = *num;
            while set.contains(&shift) {
                shift += 1;
            }
            let len = shift - *num;
            if len > max {
                max = len
            }
        }
        max
    }
    // 利用快排
    pub fn longest_consecutive2(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut curlen = 1;
        let mut maxlen = 1;
        let len = nums.len();
        for i in 1..len {
            if nums[i] == nums[i - 1] + 1 {
                curlen += 1;
            } else if nums[i] == nums[i - 1] {
                continue;
            } else {
                curlen = 1;
                continue;
            }
            if maxlen < curlen {
                maxlen = curlen;
            }
        }
        maxlen
    }

    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() == 1 {
            return;
        }
        let mut posvec = Vec::new();
        let mut pos = 0;
        let len = nums.len();
        for i in nums.iter() {
            if *i == 0 {
                posvec.push(pos);
            }
            pos += 1;
        }
        let count = posvec.len();
        let mut shift = 0;
        for i in posvec {
            for j in i - shift..len - shift - 1 {
                nums[j] = nums[j + 1]
            }
            shift += 1;
        }
        for i in len - count..len {
            nums[i] = 0;
        }
    }
    pub fn move_zeroes2(nums: &mut Vec<i32>) {
        let mut lft = 0;
        let mut rit = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] == 0 {
                lft = i;
                rit = i + 1;
                break;
            }
        }
        if lft == rit {
            return;
        }
        while rit < nums.len() {
            if nums[rit] != 0 {
                nums[lft] = nums[rit];
                nums[rit] = 0;
                lft += 1;
            }
            rit += 1;
        }
    }
    pub fn trap3(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 0 {
            return 0;
        }
        let mut left_max = vec![0; n];
        left_max[0] = height[0];
        for i in 1..n {
            left_max[i] = left_max[i - 1].max(height[i]);
        }

        let mut right_max = vec![0; n];
        right_max[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            right_max[i] = right_max[i + 1].max(height[i]);
        }
        let mut ans = 0;
        for i in 0..n {
            ans += left_max[i].min(right_max[i]) - height[i];
        }
        ans
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        let max = height.iter().max().unwrap();
        let mut water = 0;
        for h in 1..*max + 1 {
            let mut lft = 0;
            let mut rit = 1;
            for i in 0..height.len() {
                if height[i] >= h {
                    lft = i;
                    rit = i + 1;
                    break;
                }
            }
            while rit < height.len() {
                if height[rit] >= h {
                    println!("{}:{},{} sub:{}", h, lft, rit, rit - lft - 1);
                    water += rit - lft - 1;
                    lft = rit;
                    println!("total:{}", water);
                }
                rit += 1;
            }
        }
        water as i32
    }

    pub fn trap2(height: Vec<i32>) -> i32 {
        let mut leftmax: Vec<i32> = Vec::new();
        let mut rightmax: Vec<i32> = Vec::new();
        for i in 0..height.len() {
            let mut max = height[i];
            for k in (0..i).rev() {
                if height[k] > max {
                    max = height[k];
                }
            }
            leftmax.push(max);
            let mut max = height[i];
            for k in i + 1..height.len() {
                if height[k] > max {
                    max = height[k];
                }
            }
            rightmax.push(max);
        }
        let mut water = 0;
        for i in 0..height.len() {
            let max = min(leftmax[i], rightmax[i]);
            water += max - height[i];
        }

        // println!("{:?}", leftmax);
        // println!("{:?}", rightmax);
        water
    }

    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let plen = p.len();
        let slen = s.len();
        if plen > slen {
            return vec![];
        }
        let mut pseg = [0usize; 26];
        let svec = s.as_str().chars().collect::<Vec<char>>();
        for c in p.as_str().chars() {
            let pos = c as usize - 97;
            pseg[pos] += 1;
        }
        let mut segs = Vec::new();
        for i in 0..slen - plen + 1 {
            let mut salphabet = [0usize; 26];
            for j in i..i + plen {
                let pos = svec[j] as usize - 97;
                salphabet[pos] += 1;
            }
            segs.push(salphabet);
        }

        let mut result = Vec::new();
        for pos in 0..segs.len() {
            let seg = segs[pos];
            let mut flag = true;
            for i in 0..26 {
                if pseg[i] != seg[i] {
                    flag = false;
                    break;
                }
            }
            if !flag {
                continue;
            }
            result.push(pos as i32);
        }
        result
    }

    fn find_anagrams2(s: String, p: String) -> Vec<i32> {
        let plen = p.len();
        let slen = s.len();
        if plen > slen {
            return vec![];
        }
        let mut pcmp = [0i32; 26];
        for c in p.as_bytes() {
            pcmp[(c - b'a') as usize] += 1;
        }
        let s = s.as_bytes();

        let mut result = Vec::new();
        let mut left = 0;

        for (right, &c) in s.iter().enumerate() {
            let c = (c - b'a') as usize;
            pcmp[c] -= 1;
            while pcmp[c] < 0 {
                pcmp[(s[left] - b'a') as usize] += 1;
                left += 1;
            }
            if right - left + 1 == plen {
                result.push(left as i32);
            }
        }
        result
    }

    pub fn subarray_sum2(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = HashMap::with_capacity(nums.len()); // 预分配空间
        let mut s = 0;
        let mut ans = 0;
        for x in nums {
            *cnt.entry(s).or_insert(0) += 1;
            s += x;
            if let Some(&c) = cnt.get(&(s - k)) {
                ans += c;
            }
        }
        ans
    }
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut mp = HashMap::new();
        let mut peek = 0;
        let mut count = 0;
        for (pos, num) in nums.iter().enumerate() {
            if pos == 0 {
                mp.insert(*num,1);peek = *num;
            } else {
                peek += *num;
                match mp.get(&(peek - k)) {
                    Some(v) => count += *v,
                    None => (),
                }
                mp.entry(peek).and_modify(|v| *v += 1).or_insert(1);
            }
            if peek == k { count += 1; }

        }
        count
    }

    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut prioirqueue :BinaryHeap<PosValue>= BinaryHeap::new();
        let mut lft = 0;
        let mut rit = k as usize;
        for i in 0..k as usize {
            prioirqueue.push(PosValue::new(i,nums[i]));
        }
        let mut ans = Vec::with_capacity(nums.len());
        ans.push(*prioirqueue.peek().unwrap());
        while rit < nums.len() {
            if prioirqueue.peek().unwrap().value == nums[lft] {
                prioirqueue.pop();
            }
            lft += 1;
            prioirqueue.push(PosValue::new(rit,nums[rit]));
            loop {
                if prioirqueue.peek().unwrap().pos < lft || prioirqueue.peek().unwrap().pos > rit { prioirqueue.pop(); } else { break; }
            }
            ans.push(*prioirqueue.peek().unwrap());
            rit += 1;
        }
        ans
    }

}
#[derive( Eq, PartialEq,PartialOrd)]
pub struct PosValue{
    pub pos: usize,
    pub value: i32,
}

impl Ord for PosValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
impl PosValue {
    pub fn new(pos: usize, value: i32) -> PosValue {
        PosValue{ pos, value }
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn testmax_sliding_window() {
        let (vec ,k)= (vec![1,3,-1,-3,5,3,6,7],3);
        let ans = Solution::max_sliding_window(vec, k);
        println!("ans={:?}", ans);

    }


    #[test]
    fn subarray_sum(){
        // let (nums, k) = (vec![1,1,1],2);
        // let (nums, k) = (vec![1,2,3],3);
        // let (nums, k) = (vec![1],0);
        let (nums, k) = (vec![-1,-1,1],0);
        let i = Solution::subarray_sum(nums, k);
        println!("max : {}",i);
    }

    #[test]
    fn test_find_anagrams() {
        // let (s,p) = ("cbaebabacd".to_string(),"abc".to_string())
        let (s, p) = ("acdcaeccde".to_string(), "c".to_string());

        let res = Solution::find_anagrams(s, p);
        println!("{:?}", res);
    }

    #[test]
    fn test_trap() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        // let height = vec![4,2,0,3,2,5];
        // let height = vec![2,0,2];
        let water = Solution::trap3(height);
        println!("water = {}", water);
    }

    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 0, 3, 12];
        // let mut nums = vec![0];
        // Solution::move_zeroes(&mut nums);
        Solution::move_zeroes2(&mut nums);
        println!("{:?}", nums);
    }

    #[test]
    fn test_group_anagrams() {
        let list = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let list = Solution::group_anagrams(list);
    }

    #[test]
    fn test_longest_consecutive() {
        // let num = vec![100,4,200,1,3,2];
        // let num = vec![-6,-9,8,-8,-1,-3,-6,8,-9,-1,-4,-8,-5,0,1,6,-8,-5,-7,8,-2,-8,4,5,-5,-1,-5];
        // let num = vec![-1,9,-3,-6,7,-8,-6,2,9,2,3,-2,4,-1,0,6,1,-9,6,8,6,5,2];
        let num = vec![
            -6, 6, -3, 3, 0, 8, 4, -6, -4, 2, -8, -1, -2, 9, -3, -9, 2, -9, -2, 8, 5, -7, 9, -7, 7,
            -8, 5, 8,
        ];
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
