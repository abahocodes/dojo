class Solution {
public:
    vector<int> findDisappearedNumbers(vector<int>& nums) {
        // Mark value v as seen by making nums[v - 1] negative.
        for (int x : nums) {
            int i = abs(x) - 1;
            if (nums[i] > 0) nums[i] = -nums[i];
        }
        vector<int> out;
        for (int i = 0; i < (int)nums.size(); i++) {
            if (nums[i] > 0) out.push_back(i + 1);
        }
        return out;
    }
};
