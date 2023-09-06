//! The common data structure definition for leetcode problems.

/**
The definition of `ListNode`, used by many problems.
*/
#[derive(PartialEq, Eq, Debug)]
pub(crate) struct ListNode {
  val: i32,
  next: Option<Box<Self>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    Self { next: None, val }
  }
}

trait ToListNode {
  fn to_list_node(self, reverse: bool) -> Option<Box<ListNode>>;
}

impl ToListNode for i32 {
  /// Convert a number to the list of every bit of the number.
  fn to_list_node(mut self, reverse: bool) -> Option<Box<ListNode>> {
    let mut vec = vec![];
    while self / 10 > 0 {
      vec.push(self % 10);
      self /= 10;
    }
    vec.push(self % 10);
    // the sequence of the vec is opposite of the number
    vec.to_list_node(!reverse)
  }
}

impl ToListNode for Vec<i32> {
  /// Build list node from the vector of the numbers.
  fn to_list_node(mut self, reverse: bool) -> Option<Box<ListNode>> {
    let mut next = None;
    if !reverse {
      self.reverse();
    }
    for val in self {
      next = Some(Box::new(ListNode { val, next }));
    }
    next
  }
}

trait ToVec {
  fn to_num_vec(self) -> Vec<i32>;
  fn to_node_vec(self) -> Vec<Option<Box<ListNode>>>;
}

impl ToVec for Option<Box<ListNode>> {
  /// Build the vector of the numbers from the a list node.
  fn to_num_vec(self) -> Vec<i32> {
    let (mut vec, mut temp) = (vec![], &self);
    while let Some(n) = temp {
      vec.push(n.val);
      temp = &n.next;
    }
    vec
  }

  /// Build the vector of the node from the a list node.
  fn to_node_vec(self) -> Vec<Option<Box<ListNode>>> {
    let (mut vec, mut current) = (vec![], self);
    while let Some(v) = current.as_mut() {
      // use Option::take() to take the value out of the Option, and then leaving a None in its place.
      // let node = std::mem::replace(&mut v.next, None);
      let node = v.next.take();
      vec.push(current);
      current = node;
    }
    vec
  }
}

use std::{cell::RefCell, rc::Rc};

/// The definition of a binary tree node (`ListNode`), used by many problems.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TreeNode {
  val: i32,
  left: Option<Rc<RefCell<Self>>>,
  right: Option<Rc<RefCell<Self>>>,
}

impl TreeNode {
  #[inline]
  fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }

  #[inline]
  fn new_option(val: Option<i32>) -> Option<Rc<RefCell<Self>>> {
    val.map(|v| Rc::new(RefCell::new(Self::new(v))))
  }

  /**
  Building binary tree from `Vec<Option<i32>>`, Some means valued node, None means empty node.

  For example:

  `[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]` will be transformed to:
  ```html
       1
     /   \
    2     3
   / \   /
  4   5 6
  ```

  `[Some(1), Some(2), Some(3), Some(4), None, Some(5), None, Some(6)]` will be transformed to:
  ```html
         1
       /   \
      2     3
     / \   / \
    4  N   5  N
   /
  6
  ```

  `[Some(7), Some(5), Some(11), Some(4), None, Some(8), Some(13), Some(2), None, None, None, Some(12)]` will be transformed to:
  ```html
         7
       /  \
      5    11
     / \  / \
    4  N 8   13
   / \  / \  /
  2  N N  N 12
  ```
  */
  fn from(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
    use std::collections::VecDeque;

    let mut root = None; // save the root node
    let mut nodes: VecDeque<*mut Option<Rc<RefCell<Self>>>> = Default::default(); // save the pointer to child nodes

    for v in vec {
      // use the macro to deal with child node
      macro_rules! update {
        ($node: expr) => {
          if let Some(n) = &*$node {
            // add the pointer of child node, use raw pointer to avoid the ownership check
            // save the raw pointer of child node of new tree node dosn't need UNSAFE
            nodes.push_back(&mut n.borrow_mut().left);
            nodes.push_back(&mut n.borrow_mut().right);
          }
        };
      }
      let node = Self::new_option(v); // new tree node
      if root.is_none() {
        root = node;
        update!(&root);
      } else if let Some(current) = nodes.pop_front() {
        unsafe {
          // only dereference raw pointer should under UNSAFE
          *current = node;
          update!(current);
        }
      }
    }

    root
  }
}

/// For `q15` and `q18`, check if the target is included in the **vec_list**.
fn check_vecs_contain_target(vec_list: &Vec<Vec<i32>>, target: &Vec<i32>) -> bool {
  for old_vec in vec_list {
    let mut new_vec = target.clone();
    for old_val in old_vec {
      for i in 0..new_vec.len() {
        // check target vec if have equal element in old_vec
        if old_val == &new_vec[i] {
          new_vec.remove(i);
          break;
        }
      }
    }
    // if all elemnets have been removed, mean the vec is duplicate
    if new_vec.is_empty() {
      return true;
    }
  }
  false
}

/// For `q126` and `q127`, check if two words differ by only one character.
fn check_diff_one_char(old_word: &String, new_word: &String) -> bool {
  let mut count = 0;
  let (old_u8s, new_u8s): (&[u8], &[u8]) = (old_word.as_ref(), new_word.as_ref());

  for i in 0..old_u8s.len() {
    if old_u8s[i] != new_u8s[i] {
      count += 1;
      if count > 1 {
        return false;
      }
    }
  }

  count == 1
}

/// Check element content equivalence without element order.
fn check_element_eq<T>(v1: T, v2: T) -> bool
where
  T: IntoIterator,
  T::Item: Eq + std::hash::Hash + std::fmt::Debug,
{
  use std::collections::HashMap;

  let (mut length1, mut length2) = (0, 0);
  let (mut content1, mut content2) = (HashMap::new(), HashMap::new());

  for v in v1 {
    length1 += 1;
    *content1.entry(v).or_insert(0) += 1;
  }
  for v in v2 {
    length2 += 1;
    *content2.entry(v).or_insert(0) += 1;
  }

  let eq = content1 == content2 && length1 == length2;
  if !eq {
    println!("Elements are different!\nLength 1: {length1}, Length 2: {length2}");
    println!("Content 1: {content1:?}\nContent 2: {content2:?}");
  }

  eq
}

/**
Unlike everything else in the languages, macros will remain visible in sub-modules.
Also, unlike everything else in the language, macros are only accessible after their definition.
Or use `#[macro_export]` to export the macro, then use macro with code "crate::xxx_macro_name!".
*/
macro_rules! string_vec {
  ($($content:expr),*) => {{
    let mut temp = vec![];
    $(temp.push($content.to_string());)*
    temp
  }}
}

/// Provide a macro to build TreeNode which can directly use the test case syntax in LeetCode.
macro_rules! build_tree_node {
  () => { None };
  // macro matcher type 'tt' means "a single token tree",
  // which allow a independent sub token tree for other macro usage,
  // until the current rust version (1.58),
  // only positive number or zero will be treated as a single token,
  // a negative number won't be treated as it
  ($($t:tt),*) => {{
    let mut temp = vec![];
    $(temp.push(covert_tree_node!($t));)*
    TreeNode::from(temp)
  }};
}

// Use macro to transform the input content.
macro_rules! covert_tree_node {
  (null) => {
    None
  };
  ($l:literal) => {
    Some($l)
  };
}

// normal problems
mod q1008_construct_binary_search_tree_from_preorder_traversal;
mod q102_binary_tree_level_order_traversal;
mod q103_binary_tree_zipzag_level_order_traversal;
mod q107_binary_tree_level_order_traversal_ii;
mod q10_regular_expression_matching;
mod q11_container_with_most_water;
mod q122_best_time_to_buy_and_sell_stock_ii;
mod q126_word_ladder_ii;
mod q127_word_ladder;
mod q12_integer_to_roman;
mod q16_three_sum_closest;
mod q17_letter_combinations_of_a_phone_number;
mod q18_four_sum;
mod q19_remove_nth_node_from_end_of_list;
mod q200_number_of_islands;
mod q208_implement_trie;
mod q212_word_search_ii;
mod q22_generate_parentheses;
mod q23_merge_k_sorted_lists;
mod q24_swap_nodes_in_pairs;
mod q25_reverse_nodes_in_k_group;
mod q29_divide_two_integers;
mod q2_add_two_numbers;
mod q30_substring_with_concatenation_of_all_words;
mod q31_next_permutation;
mod q32_longest_valid_parentheses;
mod q33_search_in_rotated_sorted_array;
mod q34_find_first_and_last_position_of_element_in_sorted_array;
mod q35_valid_sudoku;
mod q37_sudoku_solver;
mod q39_combination_sum;
mod q3_length_of_longest_substring;
mod q407_trapping_rain_water_ii;
mod q40_combination_sum_ii;
mod q41_first_missing_positive;
mod q42_trapping_rain_water;
mod q43_multiply_strings;
mod q44_wildcard_matching;
mod q454_four_sum_ii;
mod q45_jump_game_ii;
mod q46_permutations;
mod q47_permutations_ii;
mod q48_rotate_image;
mod q49_group_anagrams;
mod q4_find_median_sorted_arrays;
mod q50_pow_x_n;
mod q51_n_queens;
mod q525_contiguous_array;
mod q52_n_queens_ii;
mod q53_maximum_subarray;
mod q543_diameter_of_binary_tree;
mod q54_spiral_matrix;
mod q55_jump_game;
mod q56_merge_intervals;
mod q57_insert_interval;
mod q59_spiral_matrix_ii;
mod q5_longest_palindrome;
mod q60_permutation_sequence;
mod q61_rotate_list;
mod q62_unique_paths;
mod q63_unique_paths_ii;
mod q64_minimum_path_sum;
mod q65_valid_number;
mod q68_text_justification;
mod q6_zipzag_conversion;
mod q71_simplify_path;
mod q72_edit_distance;
mod q73_set_matrix_zeroes;
mod q74_search_a_2d_matrix;
mod q75_sort_colors;
mod q76_minimum_window_substring;
mod q77_combinations;
mod q78_subsets;
mod q79_word_search;
mod q7_reverse_integer;
mod q80_remove_duplicates_from_sorted_array_ii;
mod q81_search_in_rotated_sorted_array_ii;
mod q82_remove_duplicates_from_sorted_list_ii;
mod q844_backspace_string_compare;
mod q84_largest_rectangle_in_histogram;
mod q85_maximal_rectangle;
mod q86_partition_list;
mod q87_scramble_string;
mod q89_gray_code;
mod q8_my_atoi;
mod q90_subsets_ii;
mod q91_decode_ways;
mod q92_reverse_linked_list_ii;
mod q93_restore_ip_addresses;
mod q94_binary_tree_inorder_traversal;
mod q95_unique_binary_search_trees_ii;
mod q96_unique_binary_search_trees;
mod q97_interleaving_string;
mod q98_validate_binary_search_tree;
mod q99_recover_binary_search_tree;

// some extra problems can only be found in "30-Day LeetCoding Challenge"
mod day_30_leetcoding_challenge;

// mod q834_sum_of_distances_in_tree; // DNF
// mod q105_construct_binary_tree_from_preorder_and_inorder_traversal; // DNF
// mod q814_binary_tree_pruning;
// mod q173_binary_search_tree_iterator;
// mod q958_check_completeness_of_a_binary_tree;
// mod q639_decode_ways_ii; // need explain
// mod q124_binary_tree_maximum_path_sum;
// mod q221_maximal_square;
// mod q1143_longest_common_subsequence;
// mod q146_lru_cache;
// mod q201_bitwise_and_of_numbers_range;
// mod q560_subarray_sum_equals_k;
// mod q678_valid_parenthesis_string;
// mod q238_product_of_array_except_self;
// mod q1046_last_stone_weight;
// mod q155_min_stack;
// mod q876_middle_of_the_linked_list;
// mod q283_move_zeroes;
// mod q136_single_number;
// mod q202_happy_number;
// mod q328_odd_even_linked_list;
// mod q725_split_linked_list;
// mod q885_spiral_matrix_iii;
// mod q143_recoder_list;
// mod q216_combination_sum_iii;
// mod q377_combination_sum_iv;
