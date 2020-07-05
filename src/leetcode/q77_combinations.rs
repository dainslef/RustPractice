/**
 * 77. Combinations
 * https://leetcode.com/problems/combinations/
 *
 * Given two integers n and k, return all possible combinations of k numbers out of 1 ... n.
 *
 * Example:
 *
 * Input: n = 4, k = 2
 * Output:
 * [
 *   [2,4],
 *   [3,4],
 *   [2,3],
 *   [1,2],
 *   [1,3],
 *   [1,4],
 * ]
 */

/**
 * normal solution, triple traversals, time complexity O(n ^ 3)
 *
 * Runtime: 128 ms
 * Memory Usage: 28.6 MB
 */
fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
  let mut temp = (1..=n).map(|v| vec![v]).collect::<Vec<Vec<i32>>>();

  for _ in 1..k {
    let mut next = vec![]; // create and move elements cost too much time
    for nums in temp {
      for v in nums.last().unwrap() + 1..=n {
        let mut nums = nums.clone();
        nums.push(v);
        next.push(nums);
      }
    }
    temp = next; // move element
  }

  temp
}

#[test]
fn q77_test() {
  fn test(combine: impl Fn(i32, i32) -> Vec<Vec<i32>>) {
    use super::check_element_eq;

    assert!(check_element_eq(
      combine(4, 2).iter(),
      vec![
        vec![2, 4],
        vec![3, 4],
        vec![2, 3],
        vec![1, 2],
        vec![1, 3],
        vec![1, 4]
      ]
      .iter()
    ));
    assert!(check_element_eq(
      combine(5, 3).iter(),
      vec![
        vec![2, 4, 5],
        vec![2, 3, 5],
        vec![1, 2, 4],
        vec![1, 2, 5],
        vec![1, 3, 5],
        vec![2, 3, 4],
        vec![1, 3, 4],
        vec![1, 2, 3],
        vec![1, 4, 5],
        vec![3, 4, 5]
      ]
      .iter()
    ));
  }

  test(combine);
}
