class Solution {
public:
    int leastInterval(vector<string>& tasks, int n) {
        unordered_map<string, int> counts;
        for (const string& t : tasks) counts[t]++;
        int most = 0;
        for (const auto& [_, c] : counts) most = max(most, c);
        int tied = 0;
        for (const auto& [_, c] : counts) if (c == most) tied++;
        // most - 1 full rows of width n + 1, then one slot per label tied for the top count
        return max((int)tasks.size(), (most - 1) * (n + 1) + tied);
    }
};
