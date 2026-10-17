// 1010. Clone Graph (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Node {
    public int val;
    public List<Node> neighbors;

    public Node() {
        this(0);
    }

    public Node(int val) {
        this.val = val;
        this.neighbors = new ArrayList<>();
    }

    public Node(int val, List<Node> neighbors) {
        this.val = val;
        this.neighbors = neighbors;
    }
}

class Solution {
    public Node cloneGraph(Node node) {
        return null;
    }

    // dojo calls this to run the tests; you don't need to change it.
    public int[][] runCloneGraph(int[][] adjList) {
        int n = adjList.length;
        Node[] originals = new Node[n];
        for (int i = 0; i < n; i++) originals[i] = new Node(i + 1);
        for (int i = 0; i < n; i++) {
            for (int v : adjList[i]) originals[i].neighbors.add(originals[v - 1]);
        }
        Node clone = cloneGraph(n > 0 ? originals[0] : null);

        // The original graph must be left as it was.
        for (int i = 0; i < n; i++) {
            Node node = originals[i];
            if (node.val != i + 1 || node.neighbors.size() != adjList[i].length) return new int[][] {{-1}};
            for (int j = 0; j < adjList[i].length; j++) {
                if (node.neighbors.get(j) != originals[adjList[i][j] - 1]) return new int[][] {{-1}};
            }
        }
        if (clone == null) return new int[0][];
        // Explore the clone: no original node, one node per value 1..n.
        Set<Node> originalSet = Collections.newSetFromMap(new IdentityHashMap<>());
        originalSet.addAll(Arrays.asList(originals));
        Set<Node> seen = Collections.newSetFromMap(new IdentityHashMap<>());
        Node[] byVal = new Node[n + 1];
        Deque<Node> stack = new ArrayDeque<>();
        stack.push(clone);
        while (!stack.isEmpty()) {
            Node node = stack.pop();
            if (originalSet.contains(node)) return new int[][] {{-1}};
            if (!seen.add(node)) continue;
            if (node.val < 1 || node.val > n || byVal[node.val] != null) return new int[][] {{-1}};
            byVal[node.val] = node;
            for (Node nb : node.neighbors) {
                if (nb == null) return new int[][] {{-1}};
                stack.push(nb);
            }
        }
        int[][] result = new int[n][];
        for (int v = 1; v <= n; v++) {
            Node node = byVal[v];
            result[v - 1] = node == null ? new int[0] : node.neighbors.stream().mapToInt(nb -> nb.val).sorted().toArray();
        }
        return result;
    }
}
