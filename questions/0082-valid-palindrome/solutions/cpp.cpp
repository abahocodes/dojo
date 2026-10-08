class Solution {
public:
    bool isPalindrome(string& s) {
        auto keep = [](unsigned char ch) { return ch < 128 && isalnum(ch); };
        int left = 0, right = (int)s.size() - 1;
        while (left < right) {
            unsigned char a = s[left], b = s[right];
            if (!keep(a)) {
                left++;
            } else if (!keep(b)) {
                right--;
            } else if (tolower(a) != tolower(b)) {
                return false;
            } else {
                left++;
                right--;
            }
        }
        return true;
    }
};
