function isBalanced(root: TreeNode | null): boolean {
    if (root === null) return true;
    // reversed pre-order puts every child before its parent
    const order: TreeNode[] = [];
    const stack: TreeNode[] = [root];
    while (stack.length > 0) {
        const node = stack.pop()!;
        order.push(node);
        if (node.left) stack.push(node.left);
        if (node.right) stack.push(node.right);
    }
    const height = new Map<TreeNode, number>();
    for (let i = order.length - 1; i >= 0; i--) {
        const node = order[i];
        const left = node.left ? height.get(node.left)! : 0;
        const right = node.right ? height.get(node.right)! : 0;
        if (Math.abs(left - right) > 1) return false;
        height.set(node, 1 + Math.max(left, right));
    }
    return true;
}
