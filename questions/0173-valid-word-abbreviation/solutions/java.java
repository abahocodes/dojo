class Solution {
    public boolean validWordAbbreviation(String word, String abbr) {
        int n = word.length();
        int m = abbr.length();
        long i = 0;
        int j = 0;
        while (i < n && j < m) {
            char c = abbr.charAt(j);
            if (Character.isDigit(c)) {
                if (c == '0') return false;
                long k = 0;
                while (j < m && Character.isDigit(abbr.charAt(j))) {
                    k = k * 10 + (abbr.charAt(j) - '0');
                    j++;
                }
                i += k;
            } else {
                if (word.charAt((int) i) != c) return false;
                i++;
                j++;
            }
        }
        return i == n && j == m;
    }
}
