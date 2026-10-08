class Solution {
public:
    int subarraySum(vector<int>& nums, int k) {
        unordered_map<int, int> seen{{0, 1}};
        int running = 0, count = 0;
        for (int x : nums) {
            running += x;
            auto it = seen.find(running - k);
            if (it != seen.end()) count += it->second;
            seen[running]++;
        }
        return count;
    }
};
