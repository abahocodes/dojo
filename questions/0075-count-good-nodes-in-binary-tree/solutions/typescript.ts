function goodNodes(root: TreeNode | null): number {
    if (root === null) return 0;
    let count = 0;
    // [node, largest value strictly above it on the path]
    const stack: [TreeNode, number][] = [[root, root.val]];
    while (stack.length > 0) {
        let [node, best] = stack.pop()!;
        if (node.val >= best) {
            count++;
            best = node.val;
        }
        if (node.left) stack.push([node.left, best]);
        if (node.right) stack.push([node.right, best]);
    }
    return count;
}
