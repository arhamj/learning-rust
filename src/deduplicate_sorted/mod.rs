pub fn solution(list: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut prev: Option<i32> = None;
    for v in list {
        if prev.is_none() || prev.unwrap() != v {
            result.push(v);
        }
        prev = Some(v);
    }
    result
}
