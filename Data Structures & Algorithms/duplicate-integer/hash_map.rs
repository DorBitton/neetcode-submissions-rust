use std::collections::HashSet;

impl Solution {
    // HashSet Solution
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();

        for num in nums {
            if !seen.insert(num) { // O(n) = O(1)
                return true;
            }
        }
        return false;

        // O(n) = O(n)
    }
}
