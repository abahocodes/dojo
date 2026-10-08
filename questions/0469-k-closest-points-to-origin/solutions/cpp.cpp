class Solution {
public:
    vector<vector<int>> kClosest(vector<vector<int>>& points, int k) {
        // Max-heap of {squared distance, index}, holding the k closest so far.
        priority_queue<pair<int, int>> heap;
        for (int i = 0; i < (int)points.size(); i++) {
            int x = points[i][0], y = points[i][1];
            int d = x * x + y * y;
            if ((int)heap.size() < k) {
                heap.push({d, i});
            } else if (d < heap.top().first) {
                heap.pop();
                heap.push({d, i});
            }
        }
        vector<vector<int>> result;
        while (!heap.empty()) {
            result.push_back(points[heap.top().second]);
            heap.pop();
        }
        return result;
    }
};
