class Solution {
    public int stringToInteger(String s) {
        int n = s.length();
        int i = 0;
        while (i < n && s.charAt(i) == ' ') i++;
        int sign = 1;
        if (i < n && (s.charAt(i) == '+' || s.charAt(i) == '-')) {
            if (s.charAt(i) == '-') sign = -1;
            i++;
        }
        long value = 0;
        while (i < n && s.charAt(i) >= '0' && s.charAt(i) <= '9') {
            value = value * 10 + (s.charAt(i) - '0');
            if (value > Integer.MAX_VALUE) { // stop early: the result is clamped anyway
                return sign == 1 ? Integer.MAX_VALUE : Integer.MIN_VALUE;
            }
            i++;
        }
        return (int) (sign * value);
    }
}
