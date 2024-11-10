use crate::leetcode::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut results = Vec::new();

        let mut current_l1 = l1;
        let mut current_l2 = l2;

        let mut overflows = false;

        loop {
            let mut sum = 0;

            if overflows {
                sum += 1;
                overflows = false;
            }

            if let Some(list_node) = current_l1 {
                sum += list_node.val;
                current_l1 = list_node.next;
            }

            if let Some(list_node) = current_l2 {
                sum += list_node.val;
                current_l2 = list_node.next;
            }

            if sum >= 10 {
                overflows = true;
                sum -= 10;
            }

            results.push(sum);

            if current_l1.is_none() && current_l2.is_none() {
                if overflows {
                    results.push(1);
                }

                break;
            }
        }

        let mut l_result: Option<Box<ListNode>> = None;

        for val in results.iter().rev() {
            l_result = Some(Box::new(ListNode {
                val: *val,
                next: l_result,
            }));
        }

        l_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::leetcode::data_structures::ListNode;

    #[allow(non_upper_case_globals)]
    static from_list_slice: fn(&[i32]) -> Option<Box<ListNode>> = ListNode::from_list_slice;
    #[allow(non_upper_case_globals)]
    static to_list_slice: fn(Option<Box<ListNode>>) -> Vec<i32> = ListNode::to_list_slice;

    #[test]
    fn test_add_two_numbers_case_one() {
        let output =
            Solution::add_two_numbers(from_list_slice(&[2, 4, 3]), from_list_slice(&[5, 6, 4]));
        let expected = from_list_slice(&[7, 0, 8]);

        assert_eq!(to_list_slice(output), to_list_slice(expected));
    }

    #[test]
    fn test_add_two_numbers_case_two() {
        let output = Solution::add_two_numbers(from_list_slice(&[0]), from_list_slice(&[0]));
        let expected = from_list_slice(&[0]);

        assert_eq!(to_list_slice(output), to_list_slice(expected));
    }

    #[test]
    fn test_add_two_numbers_case_three() {
        let output = Solution::add_two_numbers(
            from_list_slice(&[9, 9, 9, 9, 9, 9, 9]),
            from_list_slice(&[9, 9, 9, 9]),
        );
        let expected = from_list_slice(&[8, 9, 9, 9, 0, 0, 0, 1]);

        assert_eq!(to_list_slice(output), to_list_slice(expected));
    }
}
