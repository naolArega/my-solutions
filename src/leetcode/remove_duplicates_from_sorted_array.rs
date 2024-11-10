pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        assert!({
            let nums_len = nums.len();
            nums_len >= 1 && nums_len <= 30_000
        });
        assert!({
            let last_num = nums.last().unwrap_or(&i32::MIN);
            *last_num >= -100 && *last_num <= 100
        });
        assert!({
            let mut prev_num = nums.first();
            let mut is_non_decreasing_order = true;

            for idx in 1..nums.len() {
                if let Some(prev_num) = prev_num {
                    if let Some(current_num) = nums.get(idx) {
                        if !(current_num >= prev_num) {
                            is_non_decreasing_order = false;
                            break;
                        }
                    }
                }

                prev_num = nums.get(idx);
            }

            is_non_decreasing_order
        });

        let next_index = 0usize;
        let last_val = if let Some(num) = nums.first() {
            num
        } else {
            return 0;
        };

        loop {}
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_remove_duplicates_from_sorted_array_case_one() {}
}
