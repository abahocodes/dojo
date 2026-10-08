function invertTree(root: TreeNode | null): TreeNode | null {
    const stack: (TreeNode | null)[] = [root];
    while (stack.length > 0) {
        const node = stack.pop()!;
        if (node !== null) {
            [node.left, node.right] = [node.right, node.left];
            stack.push(node.left, node.right);
        }
    }
    return root;
}
