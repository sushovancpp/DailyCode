struct Solution;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        let (l, r) = Self::get_left_and_right_counts(&s);
        Self::dfs(&s, 0, l, r, &mut ans);
        ans
    }

    fn get_left_and_right_counts(s: &str) -> (usize, usize) {
        let mut l = 0;
        let mut r = 0;

        for c in s.chars() {
            if c == '(' {
                l += 1;
            } else if c == ')' {
                if l == 0 {
                    r += 1;
                } else {
                    l -= 1;
                }
            }
        }

        (l, r)
    }

    fn dfs(
        s: &str,
        start: usize,
        l: usize,
        r: usize,
        ans: &mut Vec<String>,
    ) {
        if l == 0 && r == 0 && Self::is_valid(s) {
            ans.push(s.to_string());
            return;
        }

        let chars: Vec<char> = s.chars().collect();

        for i in start..chars.len() {
            if i > start && chars[i] == chars[i - 1] {
                continue;
            }

            if l > 0 && chars[i] == '(' {
                let new_s: String = chars
                    .iter()
                    .enumerate()
                    .filter_map(|(j, &c)| {
                        if j == i { None } else { Some(c) }
                    })
                    .collect();

                Self::dfs(&new_s, i, l - 1, r, ans);
            }

            if r > 0 && chars[i] == ')' {
                let new_s: String = chars
                    .iter()
                    .enumerate()
                    .filter_map(|(j, &c)| {
                        if j == i { None } else { Some(c) }
                    })
                    .collect();

                Self::dfs(&new_s, i, l, r - 1, ans);
            }
        }
    }

    fn is_valid(s: &str) -> bool {
        let mut opened = 0;

        for c in s.chars() {
            if c == '(' {
                opened += 1;
            } else if c == ')' {
                opened -= 1;
            }

            if opened < 0 {
                return false;
            }
        }

        opened == 0
    }
}
