/**
* Given four lists A, B, C, D of integer values, compute how many tuples (i, j, k, l) there are such that A[i] + B[j] + C[k] + D[l] is zero.

* To make problem a bit easier, all A, B, C, D have same length of N where 0 ≤ N ≤ 500. All integers are in the range of -228 to 228 - 1 and the result is guaranteed to be at most 231 - 1.

* Example:

* Input:
* A = [ 1, 2]
* B = [-2,-1]
* C = [-1, 2]
* D = [ 0, 2]

* Output:
* 2

* Explanation:
* The two tuples are:
* 1. (0, 0, 0, 1) -> A[0] + B[0] + C[0] + D[1] = 1 + (-2) + (-1) + 2 = 0
* 2. (1, 1, 0, 0) -> A[1] + B[1] + C[0] + D[0] = 2 + (-1) + (-1) + 0 = 0
*/

fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
  use std::collections::HashMap;

  // use hash map to save sum as key
  let (mut map, mut count) = (HashMap::new(), 0);

  // split the vecs in args into two groups, time complexity is O(n ^ 2)
  // group1(val in a + val in b)
  for val_a in &a {
    for val_b in &b {
      let key = val_a + val_b;
      match map.get(&key) {
        Some(n) => { map.insert(key, n + 1); },
        None => { map.insert(key, 1); },
      };
    }
  }

  // group2(val in c + val in d)
  for val_c in &c {
    for val_d in &d {
      // check if the value of group2(0 - val in c - val in d) which equal the group1(val in a + val in b) exists by the key of hash map
      if let Some(n) = map.get(&(0 - val_c - val_d)) {
        count += n;
      }
    }
  }

  count
}

#[test]
fn test_four_sum_count() {
  assert_eq!(
    four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
    2
  );
  assert_eq!(
    four_sum_count(vec![-1, -1], vec![-1, 1], vec![-1, 1], vec![1, -1]),
    6
  );
}
