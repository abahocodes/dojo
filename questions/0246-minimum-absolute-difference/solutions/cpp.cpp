class Solution {
public:
    vector<vector<int>> minimumAbsDifference(vector<int>& arr) {
        vector<int> a(arr);
        sort(a.begin(), a.end());
        int best = INT_MAX;
        for (size_t i = 0; i + 1 < a.size(); i++) best = min(best, a[i + 1] - a[i]);
        vector<vector<int>> out;
        for (size_t i = 0; i + 1 < a.size(); i++) {
            if (a[i + 1] - a[i] == best) out.push_back({a[i], a[i + 1]});
        }
        return out;
    }
};
