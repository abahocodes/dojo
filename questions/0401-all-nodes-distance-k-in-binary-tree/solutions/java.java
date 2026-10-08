class Solution {
    public int[] distanceK(TreeNode root, int target, int k) {
        // Turn the tree into an undirected graph keyed by value.
        Map<Integer, List<Integer>> adj = new HashMap<>();
        adj.put(root.val, new ArrayList<>());
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            for (TreeNode child : new TreeNode[] {node.left, node.right}) {
                if (child != null) {
                    adj.get(node.val).add(child.val);
                    adj.put(child.val, new ArrayList<>(List.of(node.val)));
                    stack.push(child);
                }
            }
        }
        List<Integer> frontier = new ArrayList<>(List.of(target));
        Set<Integer> seen = new HashSet<>(frontier);
        for (int step = 0; step < k && !frontier.isEmpty(); step++) {
            List<Integer> next = new ArrayList<>();
            for (int v : frontier) {
                for (int w : adj.get(v)) {
                    if (seen.add(w)) next.add(w);
                }
            }
            frontier = next;
        }
        return frontier.stream().mapToInt(Integer::intValue).toArray();
    }
}
