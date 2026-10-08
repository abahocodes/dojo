class Solution {
    public String[] shortestUniquePrefixes(String[] words) {
        List<int[]> children = new ArrayList<>();
        List<Integer> count = new ArrayList<>();
        children.add(new int[26]);
        count.add(0);
        for (String w : words) {
            int node = 0;
            for (int i = 0; i < w.length(); i++) {
                int c = w.charAt(i) - 'a';
                if (children.get(node)[c] == 0) {
                    children.get(node)[c] = children.size();
                    children.add(new int[26]);
                    count.add(0);
                }
                node = children.get(node)[c];
                count.set(node, count.get(node) + 1);
            }
        }
        String[] result = new String[words.length];
        for (int k = 0; k < words.length; k++) {
            String w = words[k];
            int node = 0;
            int length = w.length();
            for (int i = 0; i < w.length(); i++) {
                node = children.get(node)[w.charAt(i) - 'a'];
                if (count.get(node) == 1) {
                    length = i + 1;
                    break;
                }
            }
            result[k] = w.substring(0, length);
        }
        return result;
    }
}
