class Node {
    int val;
    Node next;
    Node random;

    public Node(int val) {
        this.val = val;
        this.next = null;
        this.random = null;
    }
}

class Solution {
    public Node copyRandomList(Node head) {
        // First pass: one new node per original node.
        Map<Node, Node> copies = new HashMap<>();
        for (Node node = head; node != null; node = node.next) {
            copies.put(node, new Node(node.val));
        }
        // Second pass: wire next and random through the map (get(null) is null).
        for (Node node = head; node != null; node = node.next) {
            Node copy = copies.get(node);
            copy.next = copies.get(node.next);
            copy.random = copies.get(node.random);
        }
        return copies.get(head);
    }

    // dojo calls this to run the tests; you don't need to change it.
    public int[][] runCopyRandomList(int[][] nodes) {
        int n = nodes.length;
        Node[] originals = new Node[n];
        for (int i = 0; i < n; i++) originals[i] = new Node(nodes[i][0]);
        for (int i = 0; i < n; i++) {
            originals[i].next = i + 1 < n ? originals[i + 1] : null;
            originals[i].random = nodes[i][1] >= 0 ? originals[nodes[i][1]] : null;
        }
        Node copy = copyRandomList(n > 0 ? originals[0] : null);

        // The original list must be left as it was.
        for (int i = 0; i < n; i++) {
            Node node = originals[i];
            Node next = i + 1 < n ? originals[i + 1] : null;
            Node random = nodes[i][1] >= 0 ? originals[nodes[i][1]] : null;
            if (node.val != nodes[i][0] || node.next != next || node.random != random) {
                return new int[][] {{-1}};
            }
        }
        // Walk the copy: every node must be new, and the list must end.
        Set<Node> originalSet = Collections.newSetFromMap(new IdentityHashMap<>());
        originalSet.addAll(Arrays.asList(originals));
        Map<Node, Integer> index = new IdentityHashMap<>();
        List<Node> order = new ArrayList<>();
        for (Node node = copy; node != null; node = node.next) {
            if (originalSet.contains(node) || index.containsKey(node)) return new int[][] {{-1}};
            index.put(node, order.size());
            order.add(node);
        }
        int[][] result = new int[order.size()][];
        for (int i = 0; i < order.size(); i++) {
            Node node = order.get(i);
            if (node.random == null) {
                result[i] = new int[] {node.val, -1};
            } else if (index.containsKey(node.random)) {
                result[i] = new int[] {node.val, index.get(node.random)};
            } else {
                return new int[][] {{-1}};
            }
        }
        return result;
    }
}
