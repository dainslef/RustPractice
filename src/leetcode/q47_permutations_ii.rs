/**
 * Given a collection of numbers that might contain duplicates, return all possible unique permutations.
 *
 * Example:
 *
 * Input: [1,1,2]
 * Output:
 * [
 *   [1,1,2],
 *   [1,2,1],
 *   [2,1,1]
 * ]
 */

fn permute_unique_vec(nums: Vec<i32>) -> Vec<Vec<i32>> {
  let length = nums.len();
  let mut input_to_output = vec![(nums, vec![])];

  for _ in 0..length {
    let mut next_input_to_output = vec![];
    for (input, output) in &input_to_output {
      for i in 0..input.len() {
        let (mut next_input, mut next_output) = (input.clone(), output.clone());
        next_output.push(next_input.remove(i));
        let v = (next_input, next_output);
        // check if the "v" already exists
        if !next_input_to_output.contains(&v) {
          next_input_to_output.push(v);
        }
      }
    }
    input_to_output = next_input_to_output;
  }

  input_to_output.into_iter().map(|(_, v)| v).collect()
}

// replace Vec with HashSet, code runs faster, but cost more memory
fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
  use std::collections::HashSet;

  let length = nums.len();
  let mut input_to_output = HashSet::new();
  input_to_output.insert((nums, vec![]));

  for _ in 0..length {
    let mut next_input_to_output = HashSet::new();
    for (input, output) in &input_to_output {
      for i in 0..input.len() {
        let (mut next_input, mut next_output) = (input.clone(), output.clone());
        next_output.push(next_input.remove(i));
        // use HashSet, no need to check if the target exists
        next_input_to_output.insert((next_input, next_output));
      }
    }
    input_to_output = next_input_to_output;
  }

  input_to_output.into_iter().map(|(_, v)| v).collect()
}

#[test]
fn q47_test() {
  use super::check_element_eq;
  assert!(check_element_eq(
    permute_unique(vec![1, 1, 2]),
    vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
  ));
  assert!(check_element_eq(
    permute_unique(vec![1, 2, 1, 2]),
    vec![
      vec![1, 1, 2, 2],
      vec![1, 2, 1, 2],
      vec![1, 2, 2, 1],
      vec![2, 1, 1, 2],
      vec![2, 1, 2, 1],
      vec![2, 2, 1, 1]
    ]
  ));
}
