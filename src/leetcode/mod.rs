/**
 * Use "#[cfg(test)]" to define a test module,
 * the function in test module will only compile and run when user execute "cargo test" command.
 * Functions used by tests but not used by other modules will not receive
 * warnning message like "warning: function is never used: `...`".
 */

#[cfg(test)]
mod q2_add_two_numbers;
#[cfg(test)]
mod q3_length_of_longest_substring;
#[cfg(test)]
mod q4_find_median_sorted_arrays;
#[cfg(test)]
mod q5_longest_palindrome;
#[cfg(test)]
mod q7_reverse_integer;
#[cfg(test)]
mod q8_my_atoi;
#[cfg(test)]
mod q15_three_sum;
#[cfg(test)]
mod q16_three_sum_closest;
#[cfg(test)]
mod q18_four_sum;
#[cfg(test)]
mod q126_find_ladders;
#[cfg(test)]
mod q445_add_two_numbers_two;
#[cfg(test)]
mod q454_four_sum_two;

#[derive(PartialEq, Eq, Debug)]
struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
