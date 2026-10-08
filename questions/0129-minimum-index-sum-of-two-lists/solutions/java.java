class Solution {
    public String[] findRestaurant(String[] list1, String[] list2) {
        Map<String, Integer> index = new HashMap<>();
        for (int i = 0; i < list1.length; i++) index.put(list1[i], i);
        int best = Integer.MAX_VALUE;
        List<String> result = new ArrayList<>();
        for (int j = 0; j < list2.length; j++) {
            Integer i = index.get(list2[j]);
            if (i == null) continue;
            int total = i + j;
            if (total < best) {
                best = total;
                result.clear();
                result.add(list2[j]);
            } else if (total == best) {
                result.add(list2[j]);
            }
        }
        return result.toArray(new String[0]);
    }
}
