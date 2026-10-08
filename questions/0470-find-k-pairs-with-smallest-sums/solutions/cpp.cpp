class Solution {
public:
    vector<vector<int>> kSmallestPairs(vector<int>& nums1, vector<int>& nums2, int k) {
        // Min-heap of {sum, i, j}: tuple order gives sum, then i, then j.
        using Entry = tuple<long long, int, int>;
        priority_queue<Entry, vector<Entry>, greater<Entry>> heap;
        int rows = min(k, (int)nums1.size());
        for (int i = 0; i < rows; i++) {
            heap.push({(long long)nums1[i] + nums2[0], i, 0});
        }
        vector<vector<int>> result;
        while (!heap.empty() && (int)result.size() < k) {
            auto [sum, i, j] = heap.top();
            heap.pop();
            result.push_back({nums1[i], nums2[j]});
            if (j + 1 < (int)nums2.size()) {
                heap.push({(long long)nums1[i] + nums2[j + 1], i, j + 1});
            }
        }
        return result;
    }
};
