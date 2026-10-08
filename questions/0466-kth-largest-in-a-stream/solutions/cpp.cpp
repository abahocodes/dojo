class Solution {
public:
    vector<int> kthLargestStream(int k, vector<int>& nums, vector<int>& adds) {
        // Min-heap holding the k largest values seen so far; its top is the answer.
        priority_queue<int, vector<int>, greater<int>> heap;
        auto add = [&](int v) {
            if ((int)heap.size() < k) {
                heap.push(v);
            } else if (v > heap.top()) {
                heap.pop();
                heap.push(v);
            }
        };
        for (int v : nums) add(v);
        vector<int> result;
        result.reserve(adds.size());
        for (int v : adds) {
            add(v);
            result.push_back(heap.top());
        }
        return result;
    }
};
