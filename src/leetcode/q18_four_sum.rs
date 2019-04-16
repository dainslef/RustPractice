/**
 * Given an array nums of n integers and an integer target, are there elements a, b, c, and d in nums such that a + b + c + d = target? Find all unique quadruplets in the array which gives the sum of target.
 *
 * Note:
 *
 * The solution set must not contain duplicate quadruplets.
 *
 * Example:
 *
 * Given array nums = [1, 0, -1, 0, -2, 2], and target = 0.
 *
 * A solution set is:
 * [
 *   [-1,  0, 0, 1],
 *   [-2, -1, 1, 2],
 *   [-2,  0, 0, 2]
 * ]
 */

fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  let (mut nums, mut out_nums) = (nums, vec![]);
  let length = nums.len();
  nums.sort();

  if length >= 4 {
    for i_a in 0..length - 3 {
      let a = nums[i_a];

      for i_b in i_a + 1..length - 2 {
        let b = nums[i_b];
        let (mut start, mut end) = (i_b + 1, length - 1);

        while start < end {
          let (c, d) = (nums[start], nums[end]);
          match a + b + c + d {
            sum if sum < target => start += 1,
            sum if sum > target => end -= 1,
            _ => {
              let l = vec![a, b, c, d];
              if !super::check_vecs_contain_target(&out_nums, &l) {
                out_nums.push(l);
              }
              start += 1;
              end -= 1;
            }
          }
        }
      }
    }
  }

  out_nums
}

#[test]
fn test_four_sum() {
  assert_eq!(
    four_sum(vec![1, 0, -1, 0, -2, 2], 0),
    [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
  );
  assert_eq!(
    four_sum(vec![-4, -3, -2, -1, 0, 0, 1, 2, 3, 4], 0),
    [
      [-4, -3, 3, 4],
      [-4, -2, 2, 4],
      [-4, -1, 1, 4],
      [-4, -1, 2, 3],
      [-4, 0, 0, 4],
      [-4, 0, 1, 3],
      [-3, -2, 1, 4],
      [-3, -2, 2, 3],
      [-3, -1, 0, 4],
      [-3, -1, 1, 3],
      [-3, 0, 0, 3],
      [-3, 0, 1, 2],
      [-2, -1, 0, 3],
      [-2, -1, 1, 2],
      [-2, 0, 0, 2],
      [-1, 0, 0, 1]
    ]
  );
}
