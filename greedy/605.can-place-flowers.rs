// 605. Can Place Flowers

// You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.

// Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return true if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule and false otherwise.

// Example 1:

// Input: flowerbed = [1,0,0,0,1], n = 1
// Output: true
// Example 2:

// Input: flowerbed = [1,0,0,0,1], n = 2
// Output: false

// Constraints:

// 1 <= flowerbed.length <= 2 * 104
// flowerbed[i] is 0 or 1.
// There are no two adjacent flowers in flowerbed.
// 0 <= n <= flowerbed.length

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut m = 0;
        let mut bed = flowerbed;
        let mut i = 1;
        let len = bed.len();

        match len {
            0 => return n != 0,
            1 => return if bed[0] == 0 { n <= 1 } else { n == 0 },
            2 => return if bed[0] != bed[1] { n == 0 } else { n <= 1 },
            _ => {
                if bed[0] == 0 && bed[1] == 0 {
                    bed[0] = 1;
                    m += 1;
                }
                while i < len - 1 {
                    if bed[i - 1] != 1 && bed[i + 1] != 1 {
                        if bed[i] == 0 {
                            bed[i] = 1;
                            m += 1;
                        }
                    }
                    i += 1;
                }
                if bed[len - 2] == 0 && bed[len - 1] == 0 {
                    bed[len - 1] = 1;
                    m += 1;
                }

                return m >= n;
            }
        }
    }
}
