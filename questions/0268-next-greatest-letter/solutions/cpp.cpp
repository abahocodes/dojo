class Solution {
public:
    string nextGreatestLetter(vector<string>& letters, string& target) {
        int lo = 0, hi = (int)letters.size();
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (letters[mid] <= target) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return letters[lo % letters.size()];
    }
};
