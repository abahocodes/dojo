class Solution {
    public String reverseVowels(String s) {
        char[] chars = s.toCharArray();
        int lo = 0, hi = chars.length - 1;
        while (lo < hi) {
            if (!isVowel(chars[lo])) lo++;
            else if (!isVowel(chars[hi])) hi--;
            else {
                char tmp = chars[lo];
                chars[lo++] = chars[hi];
                chars[hi--] = tmp;
            }
        }
        return new String(chars);
    }

    private boolean isVowel(char c) {
        return "aeiouAEIOU".indexOf(c) >= 0;
    }
}
