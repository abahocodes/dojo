class Solution {
public:
    int findKthLargest(vector<int>& nums, int k) {
        // Min-heap holding the k largest values seen so far; top() is the smallest of them.
        priority_queue<int, vector<int>, greater<int>> heap(nums.begin(), nums.begin() + k);
        for (size_t i = k; i < nums.size(); i++) {
            if (nums[i] > heap.top()) {
                heap.pop();
                heap.push(nums[i]);
            }
        }
        return heap.top();
    }
};
