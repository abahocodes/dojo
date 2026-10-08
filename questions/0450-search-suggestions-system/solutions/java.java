class Solution {
    public String[][] suggestedProducts(String[] products, String searchWord) {
        String[] ordered = products.clone();
        Arrays.sort(ordered);
        String[][] result = new String[searchWord.length()][];
        int start = 0;
        for (int k = 1; k <= searchWord.length(); k++) {
            String prefix = searchWord.substring(0, k);
            // First index whose word is >= prefix; longer prefixes never move it back.
            int lo = start, hi = ordered.length;
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (ordered[mid].compareTo(prefix) < 0) lo = mid + 1;
                else hi = mid;
            }
            start = lo;
            List<String> suggestions = new ArrayList<>();
            for (int i = start; i < ordered.length && i < start + 3; i++) {
                if (!ordered[i].startsWith(prefix)) break;
                suggestions.add(ordered[i]);
            }
            result[k - 1] = suggestions.toArray(new String[0]);
        }
        return result;
    }
}
