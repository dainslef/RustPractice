/**
 * Given a set of candidate numbers (candidates) (without duplicates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.
 *
 * The same repeated number may be chosen from candidates unlimited number of times.
 *
 * Note:
 *
 * All numbers (including target) will be positive integers.
 * The solution set must not contain duplicate combinations.
 * Example 1:
 *
 * Input: candidates = [2,3,6,7], target = 7,
 * A solution set is:
 * [
 *   [7],
 *   [2,2,3]
 * ]
 * Example 2:
 *
 * Input: candidates = [2,3,5], target = 8,
 * A solution set is:
 * [
 *   [2,2,2,2],
 *   [2,3,3],
 *   [3,5]
 * ]
 */

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  let mut out = vec![];
  if candidates.is_empty() {
    return out;
  }

  let mut candidates = candidates;
  candidates.sort();

  let size = target / candidates[0] + if target % candidates[0] == 0 { 0 } else { 1 };
  let mut temp: Vec<(Vec<i32>, i32)> = vec![];

  for n in 0..size {
    let mut next = vec![];
    for i in 0..candidates.len() {
      let num = candidates[i];
      if n == 0 {
        let nums = vec![num];
        match num {
          num if num == target => out.push(nums),
          num if num < target => drop(next.push((nums, num))),
          _ => {
            candidates.truncate(i);
            break;
          }
        }
      } else {
        for (k, v) in &temp {
          let mut nums = k.clone();
          nums.push(num);
          match v + num {
            sum if sum == target => {
              nums.sort();
              if !out.contains(&nums) {
                out.push(nums);
              }
            }
            sum if sum < target => drop(next.push((nums, sum))),
            _ => {}
          }
        }
      }
    }
    temp = next;
  }

  out
}

#[test]
fn test_combination_sum() {
  test(&combination_sum);
}

fn test(combination_sum: &Fn(Vec<i32>, i32) -> Vec<Vec<i32>>) {
  use super::check_element_eq;

  assert!(check_element_eq(
    combination_sum(vec![8, 6, 7, 9], 5).iter(),
    vec![].iter()
  ));
  assert!(check_element_eq(
    combination_sum(vec![2], 1).iter(),
    vec![].iter()
  ));
  assert!(check_element_eq(
    combination_sum(vec![7, 3, 2], 18).iter(),
    vec![
      vec![2, 2, 2, 2, 2, 2, 2, 2, 2],
      vec![2, 2, 2, 2, 2, 2, 3, 3],
      vec![2, 2, 2, 2, 3, 7],
      vec![2, 2, 2, 3, 3, 3, 3],
      vec![2, 2, 7, 7],
      vec![2, 3, 3, 3, 7],
      vec![3, 3, 3, 3, 3, 3]
    ]
    .iter()
  ));
  assert!(check_element_eq(
    combination_sum(vec![2, 3, 7, 6], 7).iter(),
    vec![vec![7], vec![2, 2, 3]].iter()
  ));
  assert!(check_element_eq(
    combination_sum(vec![2, 3, 5], 8).iter(),
    vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]].iter()
  ));
}
