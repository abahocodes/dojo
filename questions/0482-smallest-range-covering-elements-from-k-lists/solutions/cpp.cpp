class Solution {
public:
    vector<int> smallestRange(vector<vector<int>>& nums) {
        using Entry = tuple<int, int, int>;  // value, list, position
        priority_queue<Entry, vector<Entry>, greater<Entry>> heap;
        int high = INT_MIN;
        for (int r = 0; r < (int)nums.size(); r++) {
            heap.push({nums[r][0], r, 0});
            high = max(high, nums[r][0]);
        }
        vector<int> best = {get<0>(heap.top()), high};
        while (true) {
            auto [low, r, c] = heap.top();
            heap.pop();
            if (high - low < best[1] - best[0]) best = {low, high};
            if (c + 1 == (int)nums[r].size()) return best;
            int next = nums[r][c + 1];
            high = max(high, next);
            heap.push({next, r, c + 1});
        }
    }
};
