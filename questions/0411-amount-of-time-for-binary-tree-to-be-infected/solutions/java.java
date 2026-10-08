class Solution {
    public int amountOfTime(TreeNode root, int start) {
        Map<TreeNode, TreeNode> parent = new HashMap<>();
        TreeNode source = null;
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            if (node.val == start) source = node;
            if (node.left != null) {
                parent.put(node.left, node);
                stack.push(node.left);
            }
            if (node.right != null) {
                parent.put(node.right, node);
                stack.push(node.right);
            }
        }

        Set<TreeNode> seen = new HashSet<>();
        seen.add(source);
        List<TreeNode> frontier = new ArrayList<>();
        frontier.add(source);
        int minutes = -1;
        while (!frontier.isEmpty()) {
            minutes++;
            List<TreeNode> next = new ArrayList<>();
            for (TreeNode node : frontier) {
                for (TreeNode neighbor : new TreeNode[] {node.left, node.right, parent.get(node)}) {
                    if (neighbor != null && seen.add(neighbor)) next.add(neighbor);
                }
            }
            frontier = next;
        }
        return minutes;
    }
}
