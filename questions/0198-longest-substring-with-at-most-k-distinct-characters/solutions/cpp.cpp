class Solution {
public:
    int lengthOfLongestSubstringKDistinct(string& s, int k) {
        vector<int> count(128, 0);
        int distinct = 0, left = 0, best = 0;
        for (int right = 0; right < (int)s.size(); right++) {
            if (count[(unsigned char)s[right]]++ == 0) distinct++;
            while (distinct > k) {
                if (--count[(unsigned char)s[left]] == 0) distinct--;
                left++;
            }
            best = max(best, right - left + 1);
        }
        return best;
    }
};
