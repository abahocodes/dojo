class Solution {
    public TreeNode[] findDuplicateSubtrees(TreeNode root) {
        Map<List<Integer>, Integer> ids = new HashMap<>(); // (left id, value, right id) -> subtree id
        Map<Integer, Integer> count = new HashMap<>();     // subtree id -> occurrences
        Map<TreeNode, Integer> nodeId = new HashMap<>();   // node -> subtree id (0 means empty)
        List<TreeNode> result = new ArrayList<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        Deque<Boolean> done = new ArrayDeque<>();
        stack.push(root);
        done.push(false);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            if (!done.pop()) {
                stack.push(node);
                done.push(true);
                if (node.right != null) {
                    stack.push(node.right);
                    done.push(false);
                }
                if (node.left != null) {
                    stack.push(node.left);
                    done.push(false);
                }
                continue;
            }
            int left = node.left == null ? 0 : nodeId.get(node.left);
            int right = node.right == null ? 0 : nodeId.get(node.right);
            List<Integer> key = List.of(left, node.val, right);
            Integer sid = ids.get(key);
            if (sid == null) {
                sid = ids.size() + 1;
                ids.put(key, sid);
            }
            nodeId.put(node, sid);
            if (count.merge(sid, 1, Integer::sum) == 2) result.add(node);
        }
        return result.toArray(new TreeNode[0]);
    }
}
