class Solution {
    public int[] treeQueries(TreeNode root, int[] queries) {
        List<TreeNode> order = new ArrayList<>();
        List<Integer> depthOf = new ArrayList<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        Deque<Integer> depths = new ArrayDeque<>();
        stack.push(root);
        depths.push(0);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            int d = depths.pop();
            order.add(node);
            depthOf.add(d);
            if (node.left != null) {
                stack.push(node.left);
                depths.push(d + 1);
            }
            if (node.right != null) {
                stack.push(node.right);
                depths.push(d + 1);
            }
        }
        int n = order.size();
        int[] depth = new int[n + 1];
        int[] height = new int[n + 1];
        int levels = 0;
        for (int i = n - 1; i >= 0; i--) {
            TreeNode node = order.get(i);
            int d = depthOf.get(i);
            depth[node.val] = d;
            levels = Math.max(levels, d + 1);
            int h = 0;
            if (node.left != null) h = height[node.left.val] + 1;
            if (node.right != null) h = Math.max(h, height[node.right.val] + 1);
            height[node.val] = h;
        }
        int[] best1 = new int[levels];
        int[] best2 = new int[levels];
        int[] owner = new int[levels];
        Arrays.fill(best1, -1);
        Arrays.fill(best2, -1);
        for (int v = 1; v <= n; v++) {
            int d = depth[v];
            int reach = d + height[v];
            if (reach > best1[d]) {
                best2[d] = best1[d];
                best1[d] = reach;
                owner[d] = v;
            } else if (reach > best2[d]) {
                best2[d] = reach;
            }
        }
        int[] answer = new int[queries.length];
        for (int i = 0; i < queries.length; i++) {
            int q = queries[i];
            int d = depth[q];
            if (owner[d] != q) answer[i] = best1[d];
            else if (best2[d] >= 0) answer[i] = best2[d];
            else answer[i] = d - 1;
        }
        return answer;
    }
}
