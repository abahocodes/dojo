class Solution {
    public int[] assignTasks(int[] servers, int[] tasks) {
        // free: {weight, index}; busy: {freeTime, weight, index}
        PriorityQueue<long[]> free = new PriorityQueue<>((x, y) ->
            x[0] != y[0] ? Long.compare(x[0], y[0]) : Long.compare(x[1], y[1]));
        PriorityQueue<long[]> busy = new PriorityQueue<>((x, y) -> {
            if (x[0] != y[0]) return Long.compare(x[0], y[0]);
            if (x[1] != y[1]) return Long.compare(x[1], y[1]);
            return Long.compare(x[2], y[2]);
        });
        for (int i = 0; i < servers.length; i++) free.add(new long[] {servers[i], i});
        int[] result = new int[tasks.length];
        long time = 0;
        for (int j = 0; j < tasks.length; j++) {
            time = Math.max(time, j);
            if (free.isEmpty()) time = Math.max(time, busy.peek()[0]);
            while (!busy.isEmpty() && busy.peek()[0] <= time) {
                long[] b = busy.poll();
                free.add(new long[] {b[1], b[2]});
            }
            long[] s = free.poll();
            result[j] = (int) s[1];
            busy.add(new long[] {time + tasks[j], s[0], s[1]});
        }
        return result;
    }
}
