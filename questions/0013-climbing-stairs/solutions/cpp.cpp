class Solution {
public:
    int climbStairs(int n) {
        int prev = 1, curr = 1; // ways to reach step 0 and step 1
        for (int i = 0; i < n - 1; i++) {
            int next = prev + curr;
            prev = curr;
            curr = next;
        }
        return curr;
    }
};
