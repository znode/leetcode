// There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.

// You are giving candies to these children subjected to the following requirements:

// Each child must have at least one candy.
// Children with a higher rating get more candies than their neighbors.
// Return the minimum number of candies you need to have to distribute the candies to the children.

// Example 1:

// Input: ratings = [1,0,2]
// Output: 5
// Explanation: You can allocate to the first, second and third child with 2, 1, 2 candies respectively.
// Example 2:

// Input: ratings = [1,2,2]
// Output: 4
// Explanation: You can allocate to the first, second and third child with 1, 2, 1 candies respectively.
// The third child gets 1 candy because it satisfies the above two conditions.

// Constraints:

// n == ratings.length
// 1 <= n <= 2 * 104
// 0 <= ratings[i] <= 2 * 104

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut candy = vec![1; n];
        let mut i = 0;
        while i < n - 1 {
            if (ratings[i + 1] > ratings[i]) && (candy[i + 1] <= candy[i]) {
                candy[i + 1] = candy[i] + 1;
            }
            i += 1;
        }
        i = n - 1;
        while i > 0 {
            if (ratings[i - 1] > ratings[i]) && (candy[i - 1] <= candy[i]) {
                candy[i - 1] = candy[i] + 1;
            }
            i = i - 1;
        }

        return candy.into_iter().sum();
    }
}
