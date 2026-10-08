class Solution {
public:
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        deque<int> dq;
        vector<int> out;
        int n = nums.size();
        for (int i = 0; i < n; i++) {
            int x = nums[i];
            while (!dq.empty() && nums[dq.back()] <= x) dq.pop_back();
            dq.push_back(i);
            if (dq.front() <= i - k) dq.pop_front();
            if (i >= k - 1) out.push_back(nums[dq.front()]);
        }
        return out;
    }
};
