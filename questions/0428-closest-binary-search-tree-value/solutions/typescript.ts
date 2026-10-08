function closestValue(root: TreeNode | null, target: number): number {
    let best = root!.val;
    let node = root;
    while (node !== null) {
        const v = node.val;
        const d = Math.abs(v - target);
        const bd = Math.abs(best - target);
        if (d < bd || (d === bd && v < best)) best = v;
        if (target < v) node = node.left;
        else if (target > v) node = node.right;
        else break;
    }
    return best;
}
