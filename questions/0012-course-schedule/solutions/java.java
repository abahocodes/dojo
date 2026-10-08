class Solution {
    public boolean canFinish(int numCourses, int[][] prerequisites) {
        List<List<Integer>> unlocks = new ArrayList<>();
        for (int i = 0; i < numCourses; i++) unlocks.add(new ArrayList<>());
        int[] indegree = new int[numCourses];
        for (int[] p : prerequisites) {
            unlocks.get(p[1]).add(p[0]);
            indegree[p[0]]++;
        }

        Deque<Integer> ready = new ArrayDeque<>();
        for (int i = 0; i < numCourses; i++) if (indegree[i] == 0) ready.add(i);
        int taken = 0;
        while (!ready.isEmpty()) {
            int current = ready.poll();
            taken++;
            for (int nxt : unlocks.get(current)) {
                if (--indegree[nxt] == 0) ready.add(nxt);
            }
        }
        return taken == numCourses;
    }
}
