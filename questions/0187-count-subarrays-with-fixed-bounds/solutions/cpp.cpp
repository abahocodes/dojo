class Solution {
public:
    long long countSubarraysFixedBounds(vector<int>& nums, int minK, int maxK) {
        long long total = 0;
        int bad = -1, lastMin = -1, lastMax = -1;
        for (int i = 0; i < (int)nums.size(); i++) {
            int v = nums[i];
            if (v < minK || v > maxK) bad = i;
            if (v == minK) lastMin = i;
            if (v == maxK) lastMax = i;
            total += max(0, min(lastMin, lastMax) - bad);
        }
        return total;
    }
};
