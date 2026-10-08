class Solution {
public:
    vector<int> nextGreaterElement(vector<int>& nums1, vector<int>& nums2) {
        unordered_map<int, int> next;
        vector<int> stack; // decreasing values awaiting a greater one
        for (int x : nums2) {
            while (!stack.empty() && stack.back() < x) {
                next[stack.back()] = x;
                stack.pop_back();
            }
            stack.push_back(x);
        }
        vector<int> result;
        result.reserve(nums1.size());
        for (int x : nums1) {
            auto it = next.find(x);
            result.push_back(it == next.end() ? -1 : it->second);
        }
        return result;
    }
};
