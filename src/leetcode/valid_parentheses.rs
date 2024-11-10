pub struct Solution;

static OPENING: [char; 3] = ['(', '{', '['];
static CLOSING: [char; 3] = [')', '}', ']'];

enum ParenthesesType {
    Opening(char),
    Closing(char),
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        assert!(s.len() >= 1 && s.len() <= 10_000);
        assert!(s
            .chars()
            .all(|p| OPENING.contains(&p) || CLOSING.contains(&p)));

        if s.len() == 1 {
            return false;
        }

        let mut p_types = Vec::with_capacity(s.len() / 2);

        for p in s.chars() {
            let current_p_type = Self::p_type(&p);

            match current_p_type {
                ParenthesesType::Opening(_) => p_types.push(current_p_type),
                ParenthesesType::Closing(_) => {
                    let previous_p = match p_types.pop() {
                        Some(ParenthesesType::Opening(previouse_p)) => previouse_p,
                        _ => return false,
                    };

                    if !Self::is_valid_pair(&previous_p, &p) {
                        return false;
                    }
                }
            }
        }

        if p_types.len() > 0 {
            return false;
        }

        true
    }

    fn p_type(p: &char) -> ParenthesesType {
        if OPENING.contains(p) {
            ParenthesesType::Opening(*p)
        } else {
            ParenthesesType::Closing(*p)
        }
    }

    fn is_valid_pair(op: &char, cp: &char) -> bool {
        let Some(opening_index) = OPENING.iter().position(|p| p == op) else {
            return false;
        };
        let Some(closing_index) = CLOSING.iter().position(|p| p == cp) else {
            return false;
        };

        if opening_index == closing_index {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_valid_parentheses_case_one() {
        let output = Solution::is_valid(String::from("()"));

        assert_eq!(output, true);
    }

    #[test]
    fn test_valid_parentheses_case_two() {
        let output = Solution::is_valid(String::from("()[]{}"));

        assert_eq!(output, true);
    }

    #[test]
    fn test_valid_parentheses_case_three() {
        let output = Solution::is_valid(String::from("(]"));

        assert_eq!(output, false);
    }

    #[test]
    fn test_valid_parentheses_case_four() {
        let output = Solution::is_valid(String::from("([])"));

        assert_eq!(output, true);
    }

    #[test]
    fn test_valid_parentheses_case_five() {
        let output = Solution::is_valid(String::from("("));

        assert_eq!(output, false);
    }

    #[test]
    fn test_valid_parentheses_case_six() {
        let output = Solution::is_valid(String::from("(("));

        assert_eq!(output, false);
    }
}
