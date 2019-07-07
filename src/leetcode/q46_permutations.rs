/**
 * Given a collection of distinct integers, return all possible permutations.
 *
 * Example:
 *
 * Input: [1,2,3]
 * Output:
 * [
 *   [1,2,3],
 *   [1,3,2],
 *   [2,1,3],
 *   [2,3,1],
 *   [3,1,2],
 *   [3,2,1]
 * ]
 */

// use recursion
fn permute_recursion(nums: Vec<i32>) -> Vec<Vec<i32>> {
  let mut temp = vec![];

  fn deal_nums(input: Vec<i32>, output: Vec<i32>, result: &mut Vec<Vec<i32>>) {
    // find all possible numbers for next index
    for i in 0..input.len() {
      let (mut next_input, mut next_output) = (input.clone(), output.clone());
      next_output.push(next_input.remove(i));
      deal_nums(next_input, next_output, result);
    }
    // check input vector, if all numbers are added, push the answer to the result
    if input.is_empty() {
      result.push(output);
    }
  }

  deal_nums(nums, vec![], &mut temp);

  temp
}

// use loop
fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
  let length = nums.len();
  let mut input_to_output = vec![(nums, vec![])];

  for _ in 0..length {
    let mut next_input_to_output = vec![];
    for (input, output) in &input_to_output {
      for i in 0..input.len() {
        let (mut next_input, mut next_output) = (input.clone(), output.clone());
        next_output.push(next_input.remove(i));
        next_input_to_output.push((next_input, next_output));
      }
    }
    input_to_output = next_input_to_output;
  }

  input_to_output.into_iter().map(|(_, v)| v).collect()
}

#[test]
fn test_permute() {
  test(&permute);
  test(&permute_recursion);
}

fn test(permute: &Fn(Vec<i32>) -> Vec<Vec<i32>>) {
  use super::check_element_eq;
  assert!(check_element_eq(
    permute(vec![1, 2, 3]),
    vec![
      vec![1, 2, 3],
      vec![1, 3, 2],
      vec![2, 1, 3],
      vec![2, 3, 1],
      vec![3, 1, 2],
      vec![3, 2, 1]
    ]
  ));
  assert!(check_element_eq(
    permute(vec![4, 2, 1, 3]),
    vec![
      vec![4, 3, 2, 1],
      vec![3, 4, 2, 1],
      vec![3, 2, 4, 1],
      vec![3, 2, 1, 4],
      vec![4, 2, 3, 1],
      vec![2, 4, 3, 1],
      vec![2, 3, 4, 1],
      vec![2, 3, 1, 4],
      vec![4, 2, 1, 3],
      vec![2, 4, 1, 3],
      vec![2, 1, 4, 3],
      vec![2, 1, 3, 4],
      vec![4, 3, 1, 2],
      vec![3, 4, 1, 2],
      vec![3, 1, 4, 2],
      vec![3, 1, 2, 4],
      vec![4, 1, 3, 2],
      vec![1, 4, 3, 2],
      vec![1, 3, 4, 2],
      vec![1, 3, 2, 4],
      vec![4, 1, 2, 3],
      vec![1, 4, 2, 3],
      vec![1, 2, 4, 3],
      vec![1, 2, 3, 4]
    ]
  ));
}
