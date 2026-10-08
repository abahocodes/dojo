class Solution {
public:
    int shortestSubarray(vector<int>& nums, int k) {
        int n = nums.size();
        vector<long long> prefix(n + 1, 0);
        for (int i = 0; i < n; i++) prefix[i + 1] = prefix[i] + nums[i];
        deque<int> starts;
        int best = n + 1;
        for (int j = 0; j <= n; j++) {
            while (!starts.empty() && prefix[j] - prefix[starts.front()] >= k) {
                best = min(best, j - starts.front());
                starts.pop_front();
            }
            while (!starts.empty() && prefix[starts.back()] >= prefix[j]) {
                starts.pop_back();
            }
            starts.push_back(j);
        }
        return best <= n ? best : -1;
    }
};
