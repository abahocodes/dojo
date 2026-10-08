class Solution {
public:
    vector<long long> rangeSums(vector<int>& nums, vector<vector<int>>& queries) {
        vector<long long> prefix(nums.size() + 1, 0);
        for (size_t i = 0; i < nums.size(); i++) prefix[i + 1] = prefix[i] + nums[i];
        vector<long long> out;
        out.reserve(queries.size());
        for (const auto& q : queries) out.push_back(prefix[q[1] + 1] - prefix[q[0]]);
        return out;
    }
};
