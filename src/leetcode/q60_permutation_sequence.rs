/**
 * 60. Permutation Sequence
 * https://leetcode.com/problems/permutation-sequence/
 *
 * The set [1,2,3,...,n] contains a total of n! unique permutations.
 *
 * By listing and labeling all of the permutations in order, we get the following sequence for n = 3:
 *
 * "123"
 * "132"
 * "213"
 * "231"
 * "312"
 * "321"
 * Given n and k, return the kth permutation sequence.
 *
 * Note:
 *
 * Given n will be between 1 and 9 inclusive.
 * Given k will be between 1 and n! inclusive.
 * Example 1:
 *
 * Input: n = 3, k = 3
 * Output: "213"
 * Example 2:
 *
 * Input: n = 4, k = 9
 * Output: "2314"
 */

// don't use the solution of q46_permutations, will be "Time Limit Exceeded"
fn get_permutation(n: i32, k: i32) -> String {
  use std::collections::VecDeque;

  // init the input values and compute the value of each step
  let (mut input, mut result, mut units) = (vec![], VecDeque::new(), VecDeque::new());
  (1..=n).fold(1, |last, current| {
    let next = last * current;
    input.push(current);
    units.push_front(next);
    next
  });

  // remove the first "unit"
  units.pop_front();

  // a array start with index 0, so the "k" needs to minus 1
  let mut remainder = k - 1;
  while let Some(unit) = units.pop_front() {
    let index = (remainder / unit) as usize;
    remainder %= unit;
    result.push_back(input.remove(index));
  }

  // add the last element of "input" to the "result"
  result.push_back(input[0]);
  result
    .into_iter()
    .fold("".into(), |v1, v2| v1 + &v2.to_string())
}

#[test]
fn test_q60() {
  assert_eq!(get_permutation(3, 3), "213");
  assert_eq!(get_permutation(4, 9), "2314");
  assert_eq!(get_permutation(9, 278082), "792346851");
}
