/**
 * Counting Elements
 * https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/528/week-1/3289/
 *
 * Given an integer array arr, count element x such that x + 1 is also in arr.
 *
 * If there're duplicates in arr, count them seperately.
 *
 *
 *
 * Example 1:
 *
 * Input: arr = [1,2,3]
 * Output: 2
 * Explanation: 1 and 2 are counted cause 2 and 3 are in arr.
 * Example 2:
 *
 * Input: arr = [1,1,3,3,5,5,7,7]
 * Output: 0
 * Explanation: No numbers are counted, cause there's no 2, 4, 6, or 8 in arr.
 * Example 3:
 *
 * Input: arr = [1,3,2,3,5,0]
 * Output: 3
 * Explanation: 0, 1 and 2 are counted cause 1, 2 and 3 are in arr.
 * Example 4:
 *
 * Input: arr = [1,1,2,2]
 * Output: 2
 * Explanation: Two 1s are counted cause 2 is in arr.
 *
 *
 * Constraints:
 *
 * 1 <= arr.length <= 1000
 * 0 <= arr[i] <= 1000
 */

fn count_elements(mut arr: Vec<i32>) -> i32 {
  if let 0 | 1 = arr.len() {
    return 0; // return immediately when arr has not enough numbers
  }

  arr.sort(); // sort arr, make the elements in the arr in order
  let (mut count, mut last_size, mut current_size) = (0, 0, 0);
  let (mut last, mut current): (Option<i32>, Option<i32>) = (None, None);

  macro_rules! compute_count {
    () => {
      if current // check two adjacent elements
        .and_then(|c| last.map(|l| c - l == 1))
        .unwrap_or(false)
      {
        count += last_size; // size means the count of elements match the "x,x-1" condition
      }
    };
  }

  for v in arr {
    macro_rules! check {
      ($state: expr, $state_size: expr) => {{
        // check if has element and count the elemnt size
        let not_match = !$state.is_none() && $state != Some(v);
        if !not_match {
          $state = Some(v);
          $state_size += 1;
        }
        not_match
      }};
    }
    // count the size of same elements, compute the size when the element isn't match current state value
    if check!(last, last_size) || check!(current, current_size) {
      compute_count!();
      last = current;
      last_size = current_size;
      current = Some(v);
      current_size = 1;
    }
  }

  compute_count!(); // compute the count for the last group of same elements
  count
}

#[test]
fn count_elements_test() {
  assert_eq!(count_elements(vec![1, 1, 2]), 2);
  assert_eq!(count_elements(vec![1, 2, 3]), 2);
  assert_eq!(count_elements(vec![1, 3, 2, 3, 5, 0]), 3);
  assert_eq!(count_elements(vec![1, 1, 2, 2]), 2);
  assert_eq!(count_elements(vec![1, 1, 1, 1, 1, 2]), 5);
  assert_eq!(count_elements(vec![1, 1, 3, 3, 5, 5, 7, 7]), 0);
}
