use std::collections::HashMap;

fn main() {
    let v = vec![2,7,11,15];
    println!("answer is {:?}",two_sum(v,9));
}

pub fn two_sum (nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut too_map:HashMap<i32,i32> = HashMap::new();
    let mut ans:Vec<i32> = Vec::new();
    for (i,val) in nums.iter().enumerate() {
        let sub_target = target - *val;
        if too_map.contains_key(&sub_target) {
            ans.push(i as i32);
            ans.push(*too_map.get(&sub_target).unwrap());
            break;
        } else {
            too_map.insert(*val,i as i32);
        }
    }
    ans
}