function rightSideView(root: TreeNode | null): number[] {
    if (root === null) return [];
    const view: number[] = [];
    let level: TreeNode[] = [root];
    while (level.length > 0) {
        view.push(level[level.length - 1].val);
        const next: TreeNode[] = [];
        for (const node of level) {
            if (node.left) next.push(node.left);
            if (node.right) next.push(node.right);
        }
        level = next;
    }
    return view;
}
