/**
 * Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.
 *
 * Each number in candidates may only be used once in the combination.
 *
 * Note:
 *
 * All numbers (including target) will be positive integers.
 * The solution set must not contain duplicate combinations.
 * Example 1:
 *
 * Input: candidates = [10,1,2,7,6,1,5], target = 8,
 * A solution set is:
 * [
 *   [1, 7],
 *   [1, 2, 5],
 *   [2, 6],
 *   [1, 1, 6]
 * ]
 * Example 2:
 *
 * Input: candidates = [2,5,2,1,2], target = 5,
 * A solution set is:
 * [
 *   [1,2,2],
 *   [5]
 * ]
 */

fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  let mut out = vec![];
  let mut candidates = candidates;
  candidates.sort();

  let mut temp = vec![]; // Vec<(nums, sum, start)>

  let mut last: Option<i32> = None;
  for i in 0..candidates.len() {
    let n = candidates[i];
    // check if the element is duplicated
    if last.map(|v| v != n).unwrap_or(true) {
      if n == target {
        out.push(vec![n]);
      } else {
        temp.push((vec![n], n, i));
      }
    }
    last = Some(n);
  }

  while !temp.is_empty() {
    let mut next = vec![];
    for (nums, sum, start) in temp {
      let mut last: Option<i32> = None;
      for i in start + 1..candidates.len() {
        let n = candidates[i];
        let sum = sum + n;
        // check if the element is duplicated
        if sum <= target && last.map(|v| v != n).unwrap_or(true) {
          let mut nums = nums.clone();
          nums.push(n);
          if sum == target {
            out.push(nums);
          } else if sum < target {
            next.push((nums, sum, i));
          }
        }
        last = Some(n);
      }
    }
    temp = next;
  }

  out
}

fn combination_sum2_recursion(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  fn recursion(
    candidates: &Vec<i32>,
    target: i32,
    start: usize,
    input: &mut Vec<i32>,
    output: &mut Vec<Vec<i32>>,
  ) {
    if target == 0 {
      output.push(input.clone());
    } else if target > 0 {
      let mut last: Option<i32> = None;
      for i in start..candidates.len() {
        let n = candidates[i];
        // check to avoid adding duplicate elements
        if last.map(|v| v != n).unwrap_or(true) {
          input.push(n);
          recursion(candidates, target - n, i + 1, input, output);
          input.pop();
        }
        last = Some(n);
      }
    }
  }

  let (input, mut out, mut candidates) = (&mut vec![], vec![], candidates);
  candidates.sort();
  recursion(&candidates, target, 0, input, &mut out);

  out
}

#[test]
fn test_q40() {
  fn test(combination_sum2: impl Fn(Vec<i32>, i32) -> Vec<Vec<i32>>) {
    use super::check_element_eq;

    assert!(check_element_eq(
      combination_sum2(vec![8, 6, 7, 9], 5).iter(),
      vec![].iter()
    ));
    assert!(check_element_eq(
      combination_sum2(vec![2], 1).iter(),
      vec![].iter()
    ));
    assert!(check_element_eq(
      combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8).iter(),
      vec![vec![1, 7], vec![1, 2, 5], vec![2, 6], vec![1, 1, 6]].iter()
    ));
    assert!(check_element_eq(
      combination_sum2(vec![2, 5, 2, 1, 2], 5).iter(),
      vec![vec![1, 2, 2], vec![5]].iter()
    ));
  }

  test(combination_sum2);
  test(combination_sum2_recursion);
}
