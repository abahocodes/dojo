class Solution {
    private Map<String, List<String>> byPrefix;
    private List<String[]> result;
    private String[] square;
    private int size;

    public String[][] wordSquares(String[] words) {
        size = words[0].length();
        // Every prefix (including the empty one) -> the words starting with it.
        byPrefix = new HashMap<>();
        for (String word : words) {
            for (int i = 0; i <= size; i++) {
                byPrefix.computeIfAbsent(word.substring(0, i), p -> new ArrayList<>()).add(word);
            }
        }
        result = new ArrayList<>();
        square = new String[size];
        backtrack(0);
        return result.toArray(new String[0][]);
    }

    private void backtrack(int k) {
        if (k == size) {
            result.add(square.clone());
            return;
        }
        // Row k must start with column k of the rows placed so far.
        StringBuilder prefix = new StringBuilder();
        for (int r = 0; r < k; r++) prefix.append(square[r].charAt(k));
        for (String word : byPrefix.getOrDefault(prefix.toString(), Collections.emptyList())) {
            square[k] = word;
            backtrack(k + 1);
        }
    }
}
