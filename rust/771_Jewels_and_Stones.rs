# https://leetcode.com/problems/jewels-and-stones/
impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let jewels: Vec<char> = j.chars().collect();
        let stones: Vec<char> = s.chars().collect();
        let mut count: i32 = 0;
        for stone in stones {
            if jewels.contains(&stone) {
                count += 1;
            }
        }
        count
    }
}
