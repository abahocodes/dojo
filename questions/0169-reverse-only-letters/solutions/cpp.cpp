class Solution {
public:
    string reverseOnlyLetters(string& s) {
        string chars = s;
        int lo = 0, hi = (int)chars.size() - 1;
        while (lo < hi) {
            if (!isLetter(chars[lo])) lo++;
            else if (!isLetter(chars[hi])) hi--;
            else swap(chars[lo++], chars[hi--]);
        }
        return chars;
    }

private:
    bool isLetter(char c) {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
    }
};
