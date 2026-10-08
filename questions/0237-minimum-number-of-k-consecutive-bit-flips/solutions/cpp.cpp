class Solution {
public:
    int minKBitFlips(vector<int>& nums, int k) {
        int n = nums.size();
        vector<int> ends(n + 1, 0);
        int active = 0, flips = 0;
        for (int i = 0; i < n; i++) {
            active ^= ends[i];
            if ((nums[i] ^ active) == 0) {
                if (i + k > n) return -1;
                flips++;
                active ^= 1;
                ends[i + k] ^= 1;
            }
        }
        return flips;
    }
};
