class Solution {
    public int[] findSubstring(String s, String[] words) {
        int L = words[0].length();
        int m = words.length;
        if ((long) m * L > s.length()) return new int[0];
        Map<String, Integer> need = new HashMap<>();
        for (String w : words) need.merge(w, 1, Integer::sum);
        List<Integer> result = new ArrayList<>();
        for (int r = 0; r < L; r++) {
            Map<String, Integer> have = new HashMap<>();
            int left = r;
            int count = 0;
            for (int right = r; right + L <= s.length(); right += L) {
                String w = s.substring(right, right + L);
                if (!need.containsKey(w)) {
                    have.clear();
                    count = 0;
                    left = right + L;
                    continue;
                }
                have.merge(w, 1, Integer::sum);
                count++;
                while (have.get(w) > need.get(w)) {
                    have.merge(s.substring(left, left + L), -1, Integer::sum);
                    count--;
                    left += L;
                }
                if (count == m) {
                    result.add(left);
                    have.merge(s.substring(left, left + L), -1, Integer::sum);
                    count--;
                    left += L;
                }
            }
        }
        Collections.sort(result);
        int[] out = new int[result.size()];
        for (int i = 0; i < out.length; i++) out[i] = result.get(i);
        return out;
    }
}
