impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        let num = num as usize;
        let sum = sum as usize;

        if sum > 9 * num {
            return "".to_string();
        }

        let nines = sum / 9;
        let remainder = sum % 9;
        
        let mut result = String::with_capacity(num);

        for _ in 0..nines {
            result.push('9');
        }

        if remainder > 0 {
            result.push(char::from_digit(remainder as u32, 10).unwrap());
        }

        while result.len() < num {
            result.push('0');
        }

        result
    }
}
