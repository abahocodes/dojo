class Solution {
    public String kthDistinct(String[] arr, int k) {
        Map<String, Integer> count = new HashMap<>();
        for (String s : arr) count.merge(s, 1, Integer::sum);
        for (String s : arr) {
            if (count.get(s) == 1 && --k == 0) return s;
        }
        return "";
    }
}
