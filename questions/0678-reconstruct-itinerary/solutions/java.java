class Solution {
    public String[] findItinerary(String[][] tickets) {
        // A min-heap per airport hands out the smallest destination first.
        Map<String, PriorityQueue<String>> graph = new HashMap<>();
        for (String[] t : tickets) {
            graph.computeIfAbsent(t[0], k -> new PriorityQueue<>()).add(t[1]);
        }

        // Hierholzer: walk greedily; an airport with no tickets left is final,
        // so it goes onto the route (built back to front).
        LinkedList<String> route = new LinkedList<>();
        Deque<String> stack = new ArrayDeque<>();
        stack.push("JFK");
        while (!stack.isEmpty()) {
            PriorityQueue<String> out = graph.get(stack.peek());
            if (out != null && !out.isEmpty()) {
                stack.push(out.poll());
            } else {
                route.addFirst(stack.pop());
            }
        }
        return route.toArray(new String[0]);
    }
}
