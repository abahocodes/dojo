class Solution {
public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        vector<int> sorted = candidates;
        sort(sorted.begin(), sorted.end());
        vector<vector<int>> result;
        vector<int> current;
        backtrack(sorted, 0, target, current, result);
        return result;
    }

private:
    void backtrack(const vector<int>& sorted, size_t start, int remaining, vector<int>& current,
                   vector<vector<int>>& result) {
        if (remaining == 0) {
            result.push_back(current);
            return;
        }
        for (size_t i = start; i < sorted.size(); i++) {
            int c = sorted[i];
            if (c > remaining) break; // sorted, so every later candidate is too big as well
            current.push_back(c);
            backtrack(sorted, i, remaining - c, current, result); // i, not i + 1: a candidate may be reused
            current.pop_back();
        }
    }
};
