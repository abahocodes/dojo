class Solution {
public:
    int maxScoreSplit(string& s) {
        int score = count(s.begin(), s.end(), '1');
        int best = 0;
        for (size_t i = 0; i + 1 < s.size(); i++) {
            score += s[i] == '0' ? 1 : -1;
            best = max(best, score);
        }
        return best;
    }
};
