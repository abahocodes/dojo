class Solution {
    public String reverseOnlyLetters(String s) {
        char[] chars = s.toCharArray();
        int lo = 0, hi = chars.length - 1;
        while (lo < hi) {
            if (!isLetter(chars[lo])) lo++;
            else if (!isLetter(chars[hi])) hi--;
            else {
                char tmp = chars[lo];
                chars[lo++] = chars[hi];
                chars[hi--] = tmp;
            }
        }
        return new String(chars);
    }

    private boolean isLetter(char c) {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
    }
}
