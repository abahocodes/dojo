function isValidBst(root: TreeNode | null): boolean {
    const stack: TreeNode[] = [];
    let prev: number | null = null;
    let node = root;
    while (stack.length > 0 || node !== null) {
        while (node !== null) {
            stack.push(node);
            node = node.left;
        }
        const top = stack.pop()!;
        if (prev !== null && top.val <= prev) return false;
        prev = top.val;
        node = top.right;
    }
    return true;
}
