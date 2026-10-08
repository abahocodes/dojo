class Solution {
    public long numDecodings(String s) {
        long prev = 1, curr = s.charAt(0) != '0' ? 1 : 0;
        for (int i = 2; i <= s.length(); i++) {
            long next = 0;
            if (s.charAt(i - 1) != '0') next += curr;
            int pair = (s.charAt(i - 2) - '0') * 10 + (s.charAt(i - 1) - '0');
            if (pair >= 10 && pair <= 26) next += prev;
            prev = curr;
            curr = next;
        }
        return curr;
    }
}
