function isSameTree(p: TreeNode | null, q: TreeNode | null): boolean {
    const stack: [TreeNode | null, TreeNode | null][] = [[p, q]];
    while (stack.length > 0) {
        const [a, b] = stack.pop()!;
        if (a === null && b === null) continue;
        if (a === null || b === null || a.val !== b.val) return false;
        stack.push([a.left, b.left], [a.right, b.right]);
    }
    return true;
}
