class Solution {
    public String orderlyQueue(String s, int k) {
        if (k == 1) {
            String doubled = s + s;
            String best = s;
            for (int i = 1; i < s.length(); i++) {
                String cand = doubled.substring(i, i + s.length());
                if (cand.compareTo(best) < 0) best = cand;
            }
            return best;
        }
        char[] chars = s.toCharArray();
        Arrays.sort(chars);
        return new String(chars);
    }
}
