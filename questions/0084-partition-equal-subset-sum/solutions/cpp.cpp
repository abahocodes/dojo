class Solution {
public:
    bool canPartition(vector<int>& nums) {
        int total = accumulate(nums.begin(), nums.end(), 0);
        if (total % 2 != 0) return false;
        int half = total / 2;
        // bit s is set when some subset weighs exactly s (sums never exceed 200 * 100)
        bitset<20001> reachable;
        reachable[0] = 1;
        for (int x : nums) reachable |= reachable << x;
        return reachable[half];
    }
};
