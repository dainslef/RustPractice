/**
 * There are two sorted arrays nums1 and nums2 of size m and n respectively.
 *
 * Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
 *
 * You may assume nums1 and nums2 cannot be both empty.
 *
 * Example 1:
 *
 * nums1 = [1, 3]
 * nums2 = [2]
 *
 * The median is 2.0
 * Example 2:
 *
 * nums1 = [1, 2]
 * nums2 = [3, 4]
 *
 * The median is (2 + 3)/2 = 2.5
 */

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
  let length = nums1.len() + nums2.len();
  let (mut temp, mut index1, mut index2) = (vec![], 0, 0);

  let mut deal_value = |v: &i32, index: &mut usize| {
    temp.push(*v);
    *index += 1;
  };

  loop {
    // put the elements which in nums1 or nums2 into new Vec
    match (nums1.get(index1), nums2.get(index2)) {
      (Some(v1), Some(v2)) => {
        if v1 < v2 {
          deal_value(v1, &mut index1);
        } else {
          deal_value(v2, &mut index2);
        }
      }
      (Some(v), None) => deal_value(v, &mut index1),
      (None, Some(v)) => deal_value(v, &mut index2),
      _ => break,
    }
  }

  // check whether the length is odd or even, then compute the median
  if length % 2 == 0 {
    (temp[length / 2 - 1] + temp[length / 2]) as f64 / 2_f64
  } else {
    temp[length / 2] as f64
  }
}

#[test]
fn test_find_median_sorted_arrays() {
  assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
  assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
  assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![2, 4]), 2.0);
  assert_eq!(find_median_sorted_arrays(vec![], vec![2, 21, 20, 30]), 20.5);
  assert_eq!(
    find_median_sorted_arrays(vec![1, 3, 6, 7, 8, 10, 20, 20, 20, 20], vec![]),
    9.0
  );
  assert_eq!(
    find_median_sorted_arrays(vec![1, 3, 6, 7, 8, 10, 20, 20, 20, 20], vec![2, 20, 20, 30]),
    15.0
  );
}
