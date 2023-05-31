pub fn solution(list: Vec<i32>, target: i32) -> usize {
    if list.is_empty() {
        return 0;
    }

    if target < list[0] {
        return 0;
    }

    let (mut left, mut right) = (0, list.len() - 1);
    while left <= right {
        let mid: usize = (left + right) / 2;
        if list[mid] > target {
            right = mid - 1;
        } else if list[mid] < target {
            left = mid + 1;
        } else {
            return mid;
        }
    }
    return left;
}
