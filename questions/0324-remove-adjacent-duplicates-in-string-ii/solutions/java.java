class Solution {
    public String removeKDuplicates(String s, int k) {
        int n = s.length();
        char[] letters = new char[n];
        int[] counts = new int[n];
        int top = -1;
        for (int i = 0; i < n; i++) {
            char ch = s.charAt(i);
            if (top >= 0 && letters[top] == ch) {
                counts[top]++;
                if (counts[top] == k) {
                    top--;
                }
            } else {
                top++;
                letters[top] = ch;
                counts[top] = 1;
            }
        }
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i <= top; i++) {
            for (int c = 0; c < counts[i]; c++) {
                sb.append(letters[i]);
            }
        }
        return sb.toString();
    }
}
