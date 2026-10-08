class Solution {
    public String[] findAllConcatenatedWords(String[] words) {
        Integer[] order = new Integer[words.length];
        for (int i = 0; i < words.length; i++) order[i] = i;
        Arrays.sort(order, (a, b) -> words[a].length() - words[b].length());
        Set<String> known = new HashSet<>();
        boolean[] found = new boolean[words.length];
        for (int i : order) {
            String w = words[i];
            if (!known.isEmpty()) {
                int n = w.length();
                boolean[] can = new boolean[n + 1];
                can[0] = true;
                for (int end = 1; end <= n; end++) {
                    for (int start = 0; start < end; start++) {
                        if (can[start] && known.contains(w.substring(start, end))) {
                            can[end] = true;
                            break;
                        }
                    }
                }
                found[i] = can[n];
            }
            known.add(w);
        }
        List<String> result = new ArrayList<>();
        for (int i = 0; i < words.length; i++) if (found[i]) result.add(words[i]);
        return result.toArray(new String[0]);
    }
}
