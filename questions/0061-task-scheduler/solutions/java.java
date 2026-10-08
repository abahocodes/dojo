class Solution {
    public int leastInterval(String[] tasks, int n) {
        Map<String, Integer> counts = new HashMap<>();
        for (String t : tasks) counts.merge(t, 1, Integer::sum);
        int most = 0;
        for (int c : counts.values()) most = Math.max(most, c);
        int tied = 0;
        for (int c : counts.values()) if (c == most) tied++;
        // most - 1 full rows of width n + 1, then one slot per label tied for the top count
        return Math.max(tasks.length, (most - 1) * (n + 1) + tied);
    }
}
