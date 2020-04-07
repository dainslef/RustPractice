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
    return 0;
  }

  arr.sort();

  let mut count = 0;
  let (mut last_size, mut current_size) = (0, 0);
  let (mut last, mut current): (Option<i32>, Option<i32>) = (None, None);

  for v in arr {
    if last.is_none() || last.map(|n| n == v).unwrap_or(false) {
      last = Some(v);
      last_size += 1;
    } else if current.is_none() || current.map(|n| n == v).unwrap_or(false) {
      current = Some(v);
      current_size += 1;
    } else {
      if current
        .and_then(|c| last.map(|l| c - l == 1))
        .unwrap_or(false)
      {
        count += last_size;
      }
      last = current;
      last_size = current_size;
      current = Some(v);
      current_size = 1;
    }
  }

  if current
    .and_then(|c| last.map(|l| c - l == 1))
    .unwrap_or(false)
  {
    count += last_size;
  }

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
