function maxPathSum(root: TreeNode | null): number {
    const order: TreeNode[] = [];
    const stack: TreeNode[] = [root!];
    while (stack.length > 0) {
        const node = stack.pop()!;
        order.push(node);
        if (node.left !== null) stack.push(node.left);
        if (node.right !== null) stack.push(node.right);
    }

    const gain = new Map<TreeNode, number>();
    let best = root!.val;
    for (let k = order.length - 1; k >= 0; k--) {
        const node = order[k];
        const left = node.left !== null ? Math.max(gain.get(node.left)!, 0) : 0;
        const right = node.right !== null ? Math.max(gain.get(node.right)!, 0) : 0;
        best = Math.max(best, node.val + left + right);
        gain.set(node, node.val + Math.max(left, right));
    }
    return best;
}
