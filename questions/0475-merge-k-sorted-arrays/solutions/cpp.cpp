class Solution {
public:
    vector<int> mergeKSortedArrays(vector<vector<int>>& arrays) {
        // Min-heap of {value, array index, position}.
        using Entry = tuple<int, int, int>;
        priority_queue<Entry, vector<Entry>, greater<Entry>> heap;
        size_t total = 0;
        for (int a = 0; a < (int)arrays.size(); a++) {
            total += arrays[a].size();
            if (!arrays[a].empty()) heap.push({arrays[a][0], a, 0});
        }
        vector<int> merged;
        merged.reserve(total);
        while (!heap.empty()) {
            auto [value, a, p] = heap.top();
            heap.pop();
            merged.push_back(value);
            if (p + 1 < (int)arrays[a].size()) heap.push({arrays[a][p + 1], a, p + 1});
        }
        return merged;
    }
};
