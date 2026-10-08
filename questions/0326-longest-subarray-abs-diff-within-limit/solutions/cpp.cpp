class Solution {
public:
    int longestSubarrayLimit(vector<int>& nums, int limit) {
        deque<int> maxq, minq;
        int left = 0;
        int best = 0;
        for (int right = 0; right < (int) nums.size(); right++) {
            int x = nums[right];
            while (!maxq.empty() && nums[maxq.back()] < x) maxq.pop_back();
            maxq.push_back(right);
            while (!minq.empty() && nums[minq.back()] > x) minq.pop_back();
            minq.push_back(right);
            while (nums[maxq.front()] - nums[minq.front()] > limit) {
                left++;
                if (maxq.front() < left) maxq.pop_front();
                if (minq.front() < left) minq.pop_front();
            }
            best = max(best, right - left + 1);
        }
        return best;
    }
};
