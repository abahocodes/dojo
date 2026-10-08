class Solution {
    static bool isVowel(char c) {
        return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
    }

public:
    int maxVowels(string& s, int k) {
        int count = 0;
        for (int i = 0; i < k; i++) if (isVowel(s[i])) count++;
        int best = count;
        for (int i = k; i < (int)s.size(); i++) {
            if (isVowel(s[i])) count++;
            if (isVowel(s[i - k])) count--;
            best = max(best, count);
        }
        return best;
    }
};
