function kthSmallest(root: TreeNode | null, k: number): number {
    const stack: TreeNode[] = [];
    let node = root;
    while (true) {
        while (node) {
            stack.push(node);
            node = node.left;
        }
        const top = stack.pop()!;
        if (--k === 0) return top.val;
        node = top.right;
    }
}
