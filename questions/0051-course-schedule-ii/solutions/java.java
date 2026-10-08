class Solution {
    public int[] findOrder(int numCourses, int[][] prerequisites) {
        List<List<Integer>> unlocks = new ArrayList<>();
        for (int i = 0; i < numCourses; i++) unlocks.add(new ArrayList<>());
        int[] indegree = new int[numCourses];
        for (int[] p : prerequisites) {
            unlocks.get(p[1]).add(p[0]);
            indegree[p[0]]++;
        }

        // A min-heap always hands out the smallest course that is ready now,
        // which gives the lexicographically smallest valid order.
        PriorityQueue<Integer> ready = new PriorityQueue<>();
        for (int i = 0; i < numCourses; i++) if (indegree[i] == 0) ready.add(i);
        int[] order = new int[numCourses];
        int count = 0;
        while (!ready.isEmpty()) {
            int current = ready.poll();
            order[count++] = current;
            for (int next : unlocks.get(current)) {
                if (--indegree[next] == 0) ready.add(next);
            }
        }
        return count == numCourses ? order : new int[0];
    }
}
