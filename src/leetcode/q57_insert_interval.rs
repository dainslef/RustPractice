/**
 * 57. Insert Interval
 * https://leetcode.com/problems/insert-interval/
 *
 * Given a set of non-overlapping intervals, insert a new interval into the intervals (merge if necessary).
 *
 * You may assume that the intervals were initially sorted according to their start times.
 *
 * Example 1:
 *
 * Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
 * Output: [[1,5],[6,9]]
 * Example 2:
 *
 * Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
 * Output: [[1,2],[3,10],[12,16]]
 * Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
 * NOTE: input types have been changed on April 15, 2019. Please reset to default code definition to get new method signature.
 */

fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
  intervals.push(new_interval);
  intervals.sort_by(|x, y| x.cmp(y));
  let mut temp: Vec<Vec<i32>> = vec![];

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
fn q57_test() {
  assert_eq!(
    insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
    [[1, 5], [6, 9]]
  );
  assert_eq!(
    insert(
      vec![
        vec![1, 2],
        vec![3, 5],
        vec![6, 7],
        vec![8, 10],
        vec![12, 16]
      ],
      vec![4, 8]
    ),
    [[1, 2], [3, 10], [12, 16]]
  );
}
