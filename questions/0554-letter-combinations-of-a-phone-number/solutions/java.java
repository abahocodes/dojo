class Solution {
    private static final String[] KEYS = {
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    };

    public String[] letterCombinations(String digits) {
        List<String> result = new ArrayList<>();
        if (!digits.isEmpty()) backtrack(digits, 0, new StringBuilder(), result);
        return result.toArray(new String[0]);
    }

    private void backtrack(String digits, int i, StringBuilder current, List<String> result) {
        if (i == digits.length()) {
            result.add(current.toString());
            return;
        }
        for (char letter : KEYS[digits.charAt(i) - '0'].toCharArray()) {
            current.append(letter);
            backtrack(digits, i + 1, current, result);
            current.deleteCharAt(current.length() - 1);
        }
    }
}
