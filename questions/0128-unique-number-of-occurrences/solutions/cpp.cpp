class Solution {
public:
    bool uniqueOccurrences(vector<int>& arr) {
        unordered_map<int, int> count;
        for (int x : arr) count[x]++;
        unordered_set<int> seen;
        for (auto& [x, c] : count) {
            if (!seen.insert(c).second) return false;
        }
        return true;
    }
};
