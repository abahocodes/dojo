class Solution {
    private String s;
    private int n;
    private boolean[][] pal;
    private List<String[]> result;
    private Deque<String> current;

    public String[][] partition(String s) {
        this.s = s;
        n = s.length();
        // pal[i][j] is true when s[i..j] is a palindrome
        pal = new boolean[n][n];
        for (int i = n - 1; i >= 0; i--) {
            for (int j = i; j < n; j++) {
                if (s.charAt(i) == s.charAt(j) && (j - i < 2 || pal[i + 1][j - 1])) {
                    pal[i][j] = true;
                }
            }
        }

        result = new ArrayList<>();
        current = new ArrayDeque<>();
        backtrack(0);
        return result.toArray(new String[0][]);
    }

    private void backtrack(int start) {
        if (start == n) {
            result.add(current.toArray(new String[0]));
            return;
        }
        for (int end = start; end < n; end++) {
            if (pal[start][end]) {
                current.addLast(s.substring(start, end + 1));
                backtrack(end + 1);
                current.removeLast();
            }
        }
    }
}
