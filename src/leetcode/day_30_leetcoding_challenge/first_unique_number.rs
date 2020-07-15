/**
 * First Unique Number
 * https://leetcode.com/explore/featured/card/30-day-leetcoding-challenge/531/week-4/3313/
 *
 * You have a queue of integers, you need to retrieve the first unique integer in the queue.
 *
 * Implement the FirstUnique class:
 *
 * FirstUnique(int[] nums) Initializes the object with the numbers in the queue.
 * int showFirstUnique() returns the value of the first unique integer of the queue, and returns -1 if there is no such integer.
 * void add(int value) insert value to the queue.
 *
 *
 * Example 1:
 *
 * Input:
 * ["FirstUnique","showFirstUnique","add","showFirstUnique","add","showFirstUnique","add","showFirstUnique"]
 * [[[2,3,5]],[],[5],[],[2],[],[3],[]]
 * Output:
 * [null,2,null,2,null,3,null,-1]
 *
 * Explanation:
 * FirstUnique firstUnique = new FirstUnique([2,3,5]);
 * firstUnique.showFirstUnique(); // return 2
 * firstUnique.add(5);            // the queue is now [2,3,5,5]
 * firstUnique.showFirstUnique(); // return 2
 * firstUnique.add(2);            // the queue is now [2,3,5,5,2]
 * firstUnique.showFirstUnique(); // return 3
 * firstUnique.add(3);            // the queue is now [2,3,5,5,2,3]
 * firstUnique.showFirstUnique(); // return -1
 *
 * Example 2:
 *
 * Input:
 * ["FirstUnique","showFirstUnique","add","add","add","add","add","showFirstUnique"]
 * [[[7,7,7,7,7,7]],[],[7],[3],[3],[7],[17],[]]
 * Output:
 * [null,-1,null,null,null,null,null,17]
 *
 * Explanation:
 * FirstUnique firstUnique = new FirstUnique([7,7,7,7,7,7]);
 * firstUnique.showFirstUnique(); // return -1
 * firstUnique.add(7);            // the queue is now [7,7,7,7,7,7,7]
 * firstUnique.add(3);            // the queue is now [7,7,7,7,7,7,7,3]
 * firstUnique.add(3);            // the queue is now [7,7,7,7,7,7,7,3,3]
 * firstUnique.add(7);            // the queue is now [7,7,7,7,7,7,7,3,3,7]
 * firstUnique.add(17);           // the queue is now [7,7,7,7,7,7,7,3,3,7,17]
 * firstUnique.showFirstUnique(); // return 17
 *
 * Example 3:
 *
 * Input:
 * ["FirstUnique","showFirstUnique","add","showFirstUnique"]
 * [[[809]],[],[809],[]]
 * Output:
 * [null,809,null,-1]
 *
 * Explanation:
 * FirstUnique firstUnique = new FirstUnique([809]);
 * firstUnique.showFirstUnique(); // return 809
 * firstUnique.add(809);          // the queue is now [809,809]
 * firstUnique.showFirstUnique(); // return -1
 *
 *
 *
 * Constraints:
 *
 * 1 <= nums.length <= 10^5
 * 1 <= nums[i] <= 10^8
 * 1 <= value <= 10^8
 * At most 50000 calls will be made to showFirstUnique and add.
 */
use std::collections::{HashSet, VecDeque};

// some simple soultion might be TLE (like use HashMap to save all the num count)
struct FirstUnique {
  nums: VecDeque<i32>,
  unique_set: HashSet<i32>,
  non_unique_set: HashSet<i32>,
}

impl FirstUnique {
  fn new(nums: Vec<i32>) -> Self {
    let mut unique_set: HashSet<i32> = Default::default();
    let mut non_unique_set = HashSet::new();
    for v in &nums {
      if unique_set.contains(&v) {
        unique_set.remove(&v);
        non_unique_set.insert(*v);
      } else if !non_unique_set.contains(&v) {
        unique_set.insert(*v);
      }
    }
    FirstUnique {
      nums: VecDeque::from(nums),
      unique_set,
      non_unique_set,
    }
  }

  fn show_first_unique(&mut self) -> i32 {
    while let Some(v) = self.nums.front() {
      if self.non_unique_set.contains(v) {
        self.nums.pop_front(); // remove char which is in the Unique Set
      } else {
        break;
      }
    }
    *self.nums.front().unwrap_or(&-1)
  }

  fn add(&mut self, value: i32) {
    if self.unique_set.contains(&value) {
      self.unique_set.remove(&value);
      self.non_unique_set.insert(value);
    } else if !self.non_unique_set.contains(&value) {
      self.unique_set.insert(value);
      self.nums.push_back(value);
    }
  }
}

#[test]
fn test_first_unique_number() {
  let mut first_unique = FirstUnique::new(vec![7, 7, 7, 7, 7, 7]);
  assert_eq!(first_unique.show_first_unique(), -1);
  first_unique.add(7); // the queue is now [7,7,7,7,7,7,7]
  first_unique.add(3); // the queue is now [7,7,7,7,7,7,7,3]
  first_unique.add(3); // the queue is now [7,7,7,7,7,7,7,3,3]
  first_unique.add(7); // the queue is now [7,7,7,7,7,7,7,3,3,7]
  first_unique.add(17); // the queue is now [7,7,7,7,7,7,7,3,3,7,17]
  assert_eq!(first_unique.show_first_unique(), 17);

  let mut first_unique = FirstUnique::new(vec![2, 3, 5]);
  assert_eq!(first_unique.show_first_unique(), 2);
  first_unique.add(5); // the queue is now [2,3,5,5]
  assert_eq!(first_unique.show_first_unique(), 2);
  first_unique.add(2); // the queue is now [2,3,5,5,2]
  assert_eq!(first_unique.show_first_unique(), 3);
  first_unique.add(3); // the queue is now [2,3,5,5,2,3]
  assert_eq!(first_unique.show_first_unique(), -1);
}
