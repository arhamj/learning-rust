mod deduplicate_sorted;
mod max_sub_array;
mod merge_sorted;
mod palindrome;
mod search_insert;
mod socket;
mod two_sum;
mod valid_parenthesis;

fn main() {
    // Testing two_sum
    println!("Testing two_sum:");
    {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let result = two_sum::solution(nums, target);
        if result.len() == 2 {
            println!("{:?}", result);
        } else {
            println!("No solution found!");
        }
    }

    // Testing palindrome
    println!("Testing palindrome:");
    {
        let num = 121;
        let result = palindrome::solution(num);
        println!("Is {} a palindrome? {}", num, result);
    }

    // Testing valid parenthesis
    println!("Testing valid parenthesis:");
    {
        let s = String::from("()[]{}");
        let s_copy = s.clone();
        let result = valid_parenthesis::solution(s);
        println!("Is {} valid? {}", s_copy, result);
    }

    // Testing merge sorted
    println!("Testing merge sorted:");
    {
        let list1 = vec![1, 2, 4];
        let list2 = vec![1, 3];
        let result = merge_sorted::solution(list1, list2);
        println!("Merged list: {:?}", result);
    }

    // Testing deduplicate sorted
    println!("Testing deduplicate sorted:");
    {
        let list = vec![1, 1, 2, 3, 3, 4, 5, 5];
        let result = deduplicate_sorted::solution(list);
        println!("Deduplicated list: {:?}", result);
    }

    // Testing search insert
    println!("Testing search insert:");
    {
        let list = vec![1, 3, 5, 7];
        let target = 9;
        let result = search_insert::solution(list, target);
        println!("Insert position: {}", result);
    }

    // Testing max sub array
    println!("Testing max sub array:");
    {
        let list = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let result = max_sub_array::solution(list);
        println!("Max sub array: {:?}", result);
    }
}
