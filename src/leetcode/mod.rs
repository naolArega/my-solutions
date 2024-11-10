pub mod add_two_numbers;
pub mod fizzbuzz;
pub mod k_closest;
pub mod longest_common_prefix;
pub mod merge_intervals;
pub mod merge_two_sorted_lists;
pub mod palindrome_number;
pub mod roman_to_integer;
pub mod smaller_numbers;
pub mod sort_colors;
pub mod sorting_the_sentence;
pub mod target_indices;
pub mod two_sum;
pub mod valid_parentheses;

mod data_structures {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }

    impl ListNode {
        pub fn from_list_slice(list: &[i32]) -> Option<Box<ListNode>> {
            let mut node = None;

            for num in list.iter().rev() {
                node = Some(Box::new(ListNode {
                    val: *num,
                    next: node,
                }));
            }

            node
        }

        pub fn to_list_slice(list: Option<Box<ListNode>>) -> Vec<i32> {
            let mut num = Vec::new();
            let mut node = list;

            while let Some(inner_node) = node {
                num.push(inner_node.val);

                node = inner_node.next;
            }

            num
        }
    }
}
