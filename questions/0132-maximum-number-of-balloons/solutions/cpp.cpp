class Solution {
public:
    int maxNumberOfBalloons(string& text) {
        int have[26] = {0}, need[26] = {0};
        for (char c : text) have[c - 'a']++;
        for (char c : string("balloon")) need[c - 'a']++;
        int best = INT_MAX;
        for (int i = 0; i < 26; i++) {
            if (need[i] > 0) best = min(best, have[i] / need[i]);
        }
        return best;
    }
};
