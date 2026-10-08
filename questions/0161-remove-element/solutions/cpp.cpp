class Solution {
public:
    vector<int> removeElement(vector<int>& nums, int val) {
        int write = 0;
        for (int x : nums) {
            if (x != val) nums[write++] = x;
        }
        nums.resize(write);
        return nums;
    }
};
