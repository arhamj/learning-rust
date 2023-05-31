pub fn solution(list: Vec<i32>) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);
    if list.is_empty() {
        return vec![];
    }
    let mut max_sum: i32 = std::f32::NEG_INFINITY as i32;
    let mut prev_sum: i32 = 0;
    for (idx, elem) in list.iter().enumerate() {
        prev_sum += elem;
        if prev_sum > max_sum {
            max_sum = prev_sum;
            j = idx;
        }
        if *elem > max_sum {
            max_sum = *elem;
            prev_sum = *elem;
            i = idx;
            j = idx;
        }
    }
    return list[i..=j].to_vec();
}
