class Solution {
    public String compress(String chars) {
        StringBuilder out = new StringBuilder();
        int n = chars.length();
        int i = 0;
        while (i < n) {
            int j = i;
            while (j < n && chars.charAt(j) == chars.charAt(i)) j++;
            out.append(chars.charAt(i));
            if (j - i > 1) out.append(j - i);
            i = j;
        }
        return out.toString();
    }
}
