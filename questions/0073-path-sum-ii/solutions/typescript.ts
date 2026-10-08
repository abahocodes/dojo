function pathSum(root: TreeNode | null, targetSum: number): number[][] {
    const result: number[][] = [];
    if (root === null) return result;
    const path: number[] = [];
    let total = 0;
    // [node, leaving]: leaving=true means "undo this node on the way back up"
    const stack: [TreeNode, boolean][] = [[root, false]];
    while (stack.length > 0) {
        const [node, leaving] = stack.pop()!;
        if (leaving) {
            path.pop();
            total -= node.val;
            continue;
        }
        path.push(node.val);
        total += node.val;
        stack.push([node, true]);
        if (node.left === null && node.right === null) {
            if (total === targetSum) result.push([...path]);
        } else {
            // push right first so the left subtree is explored first
            if (node.right) stack.push([node.right, false]);
            if (node.left) stack.push([node.left, false]);
        }
    }
    return result;
}
