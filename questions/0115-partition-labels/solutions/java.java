class Solution {
    public int[] partitionLabels(String s) {
        Map<Character, Integer> last = new HashMap<>();
        for (int i = 0; i < s.length(); i++) last.put(s.charAt(i), i);
        List<Integer> sizes = new ArrayList<>();
        int start = 0, end = 0;
        for (int i = 0; i < s.length(); i++) {
            end = Math.max(end, last.get(s.charAt(i)));
            if (i == end) {
                sizes.add(i - start + 1);
                start = i + 1;
            }
        }
        return sizes.stream().mapToInt(Integer::intValue).toArray();
    }
}
