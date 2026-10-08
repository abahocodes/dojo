class Solution {
public:
    int jump(vector<int>& nums) {
        int jumps = 0, end = 0, furthest = 0;
        for (int i = 0; i + 1 < (int)nums.size(); i++) {
            furthest = max(furthest, i + nums[i]);
            if (i == end) {
                jumps++;
                end = furthest;
            }
        }
        return jumps;
    }
};
