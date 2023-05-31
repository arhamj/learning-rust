pub fn solution(orig: i32) -> bool {
    let (mut num, mut reversed_num) = (orig, 0);

    while num > 0 {
        reversed_num = reversed_num * 10 + num % 10;
        num /= 10;
    }

    reversed_num == orig
}
