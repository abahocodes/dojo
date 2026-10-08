class Solution {
    public boolean[] camelMatch(String[] queries, String pattern) {
        boolean[] result = new boolean[queries.length];
        for (int k = 0; k < queries.length; k++) result[k] = matches(queries[k], pattern);
        return result;
    }

    private boolean matches(String query, String pattern) {
        int j = 0;
        for (int i = 0; i < query.length(); i++) {
            char c = query.charAt(i);
            if (j < pattern.length() && c == pattern.charAt(j)) j++;
            else if (c >= 'A' && c <= 'Z') return false;
        }
        return j == pattern.length();
    }
}
