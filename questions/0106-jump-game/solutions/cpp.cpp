class Solution {
public:
    bool canJump(vector<int>& nums) {
        int furthest = 0;
        int last = (int)nums.size() - 1;
        for (int i = 0; i < (int)nums.size(); i++) {
            if (i > furthest) return false;
            furthest = max(furthest, i + nums[i]);
            if (furthest >= last) return true;
        }
        return true;
    }
};
