class Solution {
public:
    int equalSubstring(string& s, string& t, int maxCost) {
        int left = 0, cost = 0, best = 0;
        for (int right = 0; right < (int)s.size(); right++) {
            cost += abs(s[right] - t[right]);
            while (cost > maxCost) {
                cost -= abs(s[left] - t[left]);
                left++;
            }
            best = max(best, right - left + 1);
        }
        return best;
    }
};
