class Solution {
public:
    int minMovesComplementary(vector<int>& nums, int limit) {
        int n = nums.size();
        vector<int> delta(2 * limit + 2, 0);
        for (int i = 0; i < n / 2; i++) {
            int a = nums[i], b = nums[n - 1 - i];
            int lo = min(a, b), hi = max(a, b);
            delta[2] += 2;
            delta[lo + 1] -= 1;
            delta[hi + limit + 1] += 1;
            delta[a + b] -= 1;
            delta[a + b + 1] += 1;
        }
        int best = n, moves = 0;
        for (int t = 2; t <= 2 * limit; t++) {
            moves += delta[t];
            best = min(best, moves);
        }
        return best;
    }
};
