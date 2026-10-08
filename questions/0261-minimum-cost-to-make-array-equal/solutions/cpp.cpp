class Solution {
public:
    long long minCost(vector<int>& nums, vector<int>& cost) {
        int n = nums.size();
        vector<pair<int, int>> pairs(n);
        long long total = 0;
        for (int i = 0; i < n; i++) {
            pairs[i] = {nums[i], cost[i]};
            total += cost[i];
        }
        sort(pairs.begin(), pairs.end());
        long long acc = 0;
        long long target = pairs[0].first;
        for (auto& [value, weight] : pairs) {
            acc += weight;
            if (2 * acc >= total) {
                target = value;
                break;
            }
        }
        long long answer = 0;
        for (int i = 0; i < n; i++) answer += (long long)cost[i] * llabs(nums[i] - target);
        return answer;
    }
};
