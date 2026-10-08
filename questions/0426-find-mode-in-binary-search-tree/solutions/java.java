class Solution {
    public int[] findMode(TreeNode root) {
        List<Integer> modes = new ArrayList<>();
        int best = 0, count = 0, prev = 0;
        boolean hasPrev = false;
        Deque<TreeNode> stack = new ArrayDeque<>();
        TreeNode node = root;
        while (!stack.isEmpty() || node != null) {
            while (node != null) {
                stack.push(node);
                node = node.left;
            }
            node = stack.pop();
            count = (hasPrev && node.val == prev) ? count + 1 : 1;
            prev = node.val;
            hasPrev = true;
            if (count > best) {
                best = count;
                modes.clear();
                modes.add(node.val);
            } else if (count == best) {
                modes.add(node.val);
            }
            node = node.right;
        }
        int[] result = new int[modes.size()];
        for (int i = 0; i < result.length; i++) result[i] = modes.get(i);
        return result;
    }
}
