use std::collections::HashMap;

pub fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_idx_map: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        let expected_num = target - v;
        match num_idx_map.get(&expected_num) {
            Some(&idx) => return vec![idx, i as i32],
            None => num_idx_map.insert(*v, i as i32),
        };
    }
    return vec![];
}
