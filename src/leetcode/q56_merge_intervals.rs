/**
 * 56. Merge Intervals
 * https://leetcode.com/problems/merge-intervals/
 *
 * Given a collection of intervals, merge all overlapping intervals.
 *
 * Example 1:
 *
 * Input: [[1,3],[2,6],[8,10],[15,18]]
 * Output: [[1,6],[8,10],[15,18]]
 * Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
 * Example 2:
 *
 * Input: [[1,4],[4,5]]
 * Output: [[1,5]]
 * Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 * NOTE: input types have been changed on April 15, 2019. Please reset to default code definition to get new method signature.
 */

fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let mut temp: Vec<Vec<i32>> = vec![];
  intervals.sort_by(|x, y| x.cmp(y));

  for i in intervals {
    match temp.last_mut() {
      Some(v) if v[1] >= i[0] && v[1] < i[1] => v[1] = i[1],
      Some(v) if v[1] >= i[1] => {}
      _ => temp.push(i),
    }
  }

  temp
}

#[test]
fn q56_test() {
  assert_eq!(
    merge(vec![vec![2, 20], vec![1, 3], vec![15, 18]]),
    [[1, 20]]
  );
  assert_eq!(merge(vec![vec![3, 10], vec![0, 4]]), [[0, 10]]);
  assert_eq!(merge(vec![vec![1, 4], vec![0, 4]]), [[0, 4]]);
  assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), [[1, 5]]);
  assert_eq!(
    merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
    [[1, 6], [8, 10], [15, 18]]
  );
}
