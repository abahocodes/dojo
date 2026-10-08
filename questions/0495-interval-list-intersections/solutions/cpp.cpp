class Solution {
public:
    vector<vector<int>> intervalIntersection(vector<vector<int>>& first, vector<vector<int>>& second) {
        vector<vector<int>> result;
        size_t i = 0, j = 0;
        while (i < first.size() && j < second.size()) {
            int lo = max(first[i][0], second[j][0]);
            int hi = min(first[i][1], second[j][1]);
            if (lo <= hi) result.push_back({lo, hi});
            // The interval that ends first cannot meet anything later in the other list.
            if (first[i][1] < second[j][1]) i++;
            else j++;
        }
        return result;
    }
};
