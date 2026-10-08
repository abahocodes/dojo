class Solution {
    public int[] getOrder(int[][] tasks) {
        int n = tasks.length;
        Integer[] byEnqueue = new Integer[n];
        for (int i = 0; i < n; i++) byEnqueue[i] = i;
        Arrays.sort(byEnqueue, (a, b) ->
            tasks[a][0] != tasks[b][0] ? Integer.compare(tasks[a][0], tasks[b][0]) : Integer.compare(a, b));
        PriorityQueue<int[]> ready = new PriorityQueue<>((x, y) ->
            x[0] != y[0] ? Integer.compare(x[0], y[0]) : Integer.compare(x[1], y[1]));
        int[] order = new int[n];
        int done = 0;
        long time = 0;
        int p = 0;
        while (done < n) {
            if (ready.isEmpty() && time < tasks[byEnqueue[p]][0]) time = tasks[byEnqueue[p]][0];
            while (p < n && tasks[byEnqueue[p]][0] <= time) {
                int i = byEnqueue[p++];
                ready.add(new int[] {tasks[i][1], i});
            }
            int[] t = ready.poll();
            time += t[0];
            order[done++] = t[1];
        }
        return order;
    }
}
