class Solution {
    public int[] boundaryOfBinaryTree(TreeNode root) {
        List<Integer> result = new ArrayList<>();
        result.add(root.val);
        if (isLeaf(root)) return toArray(result);

        TreeNode node = root.left;
        while (node != null && !isLeaf(node)) {
            result.add(node.val);
            node = node.left != null ? node.left : node.right;
        }

        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode cur = stack.pop();
            if (isLeaf(cur)) {
                result.add(cur.val);
                continue;
            }
            if (cur.right != null) stack.push(cur.right);
            if (cur.left != null) stack.push(cur.left);
        }

        List<Integer> right = new ArrayList<>();
        node = root.right;
        while (node != null && !isLeaf(node)) {
            right.add(node.val);
            node = node.right != null ? node.right : node.left;
        }
        for (int i = right.size() - 1; i >= 0; i--) result.add(right.get(i));
        return toArray(result);
    }

    private boolean isLeaf(TreeNode node) {
        return node.left == null && node.right == null;
    }

    private int[] toArray(List<Integer> list) {
        int[] out = new int[list.size()];
        for (int i = 0; i < out.length; i++) out[i] = list.get(i);
        return out;
    }
}
