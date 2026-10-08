class Solution {
public:
    string reverseVowels(string& s) {
        string chars = s;
        int lo = 0, hi = (int)chars.size() - 1;
        while (lo < hi) {
            if (!isVowel(chars[lo])) lo++;
            else if (!isVowel(chars[hi])) hi--;
            else swap(chars[lo++], chars[hi--]);
        }
        return chars;
    }

private:
    bool isVowel(char c) {
        switch (c) {
            case 'a': case 'e': case 'i': case 'o': case 'u':
            case 'A': case 'E': case 'I': case 'O': case 'U':
                return true;
            default:
                return false;
        }
    }
};
