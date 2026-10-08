class Solution {
    public String alienOrder(String[] words) {
        TreeSet<Character> letters = new TreeSet<>();
        for (String w : words) for (char ch : w.toCharArray()) letters.add(ch);
        Map<Character, Set<Character>> after = new HashMap<>();
        Map<Character, Integer> indegree = new HashMap<>();
        for (char ch : letters) {
            after.put(ch, new HashSet<>());
            indegree.put(ch, 0);
        }
        for (int k = 0; k + 1 < words.length; k++) {
            String first = words[k], second = words[k + 1];
            int len = Math.min(first.length(), second.length());
            boolean differs = false;
            for (int i = 0; i < len; i++) {
                char a = first.charAt(i), b = second.charAt(i);
                if (a != b) {
                    if (after.get(a).add(b)) indegree.merge(b, 1, Integer::sum);
                    differs = true;
                    break;
                }
            }
            if (!differs && first.length() > second.length()) return ""; // a longer word sits before its own prefix
        }
        PriorityQueue<Character> heap = new PriorityQueue<>();
        for (char ch : letters) if (indegree.get(ch) == 0) heap.add(ch);
        StringBuilder order = new StringBuilder();
        while (!heap.isEmpty()) {
            char ch = heap.poll();
            order.append(ch);
            for (char nxt : after.get(ch)) {
                int left = indegree.merge(nxt, -1, Integer::sum);
                if (left == 0) heap.add(nxt);
            }
        }
        return order.length() == letters.size() ? order.toString() : "";
    }
}
