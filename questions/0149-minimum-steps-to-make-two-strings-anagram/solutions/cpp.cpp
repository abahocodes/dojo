class Solution {
public:
    int minSteps(string& s, string& t) {
        int diff[26] = {0};
        for (char c : s) diff[c - 'a']++;
        for (char c : t) diff[c - 'a']--;
        int steps = 0;
        for (int d : diff) if (d > 0) steps += d;
        return steps;
    }
};
