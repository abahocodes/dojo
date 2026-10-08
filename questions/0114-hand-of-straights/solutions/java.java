class Solution {
    public boolean isNStraightHand(int[] hand, int groupSize) {
        if (hand.length % groupSize != 0) return false;
        TreeMap<Integer, Integer> count = new TreeMap<>();
        for (int x : hand) count.merge(x, 1, Integer::sum);
        for (int x : count.keySet()) {
            int c = count.get(x);
            if (c == 0) continue;
            for (int v = x; v < x + groupSize; v++) {
                int have = count.getOrDefault(v, 0);
                if (have < c) return false;
                count.put(v, have - c);
            }
        }
        return true;
    }
}
