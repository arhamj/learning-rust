pub fn solution(list1: Vec<i32>, list2: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let (mut i, mut j) = (0, 0);
    while i < list1.len() && j < list2.len() {
        if list1[i] > list2[j] {
            result.push(list2[j]);
            j += 1;
        } else {
            result.push(list1[i]);
            i += 1;
        }
    }
    while i < list1.len() {
        result.push(list1[i]);
        i += 1;
    }
    while j < list2.len() {
        result.push(list2[j]);
        j += 1;
    }
    result
}
