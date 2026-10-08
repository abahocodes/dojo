class Solution {
public:
    int characterReplacement(string& s, int k) {
        int counts[26] = {0};
        int maxFreq = 0;
        int left = 0;
        int n = s.size();
        for (int right = 0; right < n; right++) {
            int idx = s[right] - 'A';
            counts[idx]++;
            if (counts[idx] > maxFreq) maxFreq = counts[idx];
            if (right - left + 1 - maxFreq > k) {
                counts[s[left] - 'A']--;
                left++;
            }
        }
        return n - left;
    }
};
