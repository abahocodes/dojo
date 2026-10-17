class Solution {
public:
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        vector<int> sorted = candidates;
        sort(sorted.begin(), sorted.end()); // equal values become neighbours
        vector<vector<int>> result;
        vector<int> current;
        backtrack(sorted, 0, target, current, result);
        return result;
    }

private:
    void backtrack(const vector<int>& c, int start, int remaining, vector<int>& current,
                   vector<vector<int>>& result) {
        if (remaining == 0) {
            result.push_back(current);
            return;
        }
        for (int i = start; i < (int)c.size(); i++) {
            if (c[i] > remaining) break; // every later candidate is too big as well
            if (i > start && c[i] == c[i - 1]) continue; // already tried here
            current.push_back(c[i]);
            backtrack(c, i + 1, remaining - c[i], current, result); // each card used once
            current.pop_back();
        }
    }
};
