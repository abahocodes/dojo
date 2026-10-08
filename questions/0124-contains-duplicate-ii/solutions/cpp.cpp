class Solution {
public:
    bool containsNearbyDuplicate(vector<int>& nums, int k) {
        unordered_map<int, int> last;
        for (int i = 0; i < (int)nums.size(); i++) {
            auto it = last.find(nums[i]);
            if (it != last.end() && i - it->second <= k) return true;
            last[nums[i]] = i;
        }
        return false;
    }
};
