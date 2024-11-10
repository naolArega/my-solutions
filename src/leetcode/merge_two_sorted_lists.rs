use crate::leetcode::data_structures::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut merged = Vec::new();
        let mut current_node_one = list1;
        let mut current_node_two = list2;

        while current_node_one.is_some() || current_node_two.is_some() {
            if let Some(ref node) = current_node_one {
                merged.push(node.val);
                current_node_one = node.next.clone();
            }

            if let Some(ref node) = current_node_two {
                merged.push(node.val);
                current_node_two = node.next.clone();
            }
        }

        merged.sort();
        ListNode::from_list_slice(&merged)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::leetcode::data_structures::ListNode;

    #[test]
    fn test_merge_two_sorted_lists_case_one() {
        let output = Solution::merge_two_lists(
            ListNode::from_list_slice(&[1, 2, 4]),
            ListNode::from_list_slice(&[1, 3, 4]),
        );
        let expected = ListNode::from_list_slice(&[1, 1, 2, 3, 4, 4]);

        assert_eq!(
            ListNode::to_list_slice(output),
            ListNode::to_list_slice(expected)
        );
    }

    #[test]
    fn test_merge_two_sorted_lists_case_two() {
        let output = Solution::merge_two_lists(
            ListNode::from_list_slice(&[]),
            ListNode::from_list_slice(&[]),
        );
        let expected = ListNode::from_list_slice(&[]);

        assert_eq!(
            ListNode::to_list_slice(output),
            ListNode::to_list_slice(expected)
        );
    }

    #[test]
    fn test_merge_two_sorted_lists_case_three() {
        let output = Solution::merge_two_lists(
            ListNode::from_list_slice(&[]),
            ListNode::from_list_slice(&[0]),
        );
        let expected = ListNode::from_list_slice(&[0]);

        assert_eq!(
            ListNode::to_list_slice(output),
            ListNode::to_list_slice(expected)
        );
    }

    #[test]
    fn test_merge_two_sorted_lists_case_four() {
        let output = Solution::merge_two_lists(
            ListNode::from_list_slice(&[2]),
            ListNode::from_list_slice(&[1]),
        );
        let expected = ListNode::from_list_slice(&[1, 2]);

        assert_eq!(
            ListNode::to_list_slice(output),
            ListNode::to_list_slice(expected)
        );
    }
}
