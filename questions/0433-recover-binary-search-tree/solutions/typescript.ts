function recoverTree(root: TreeNode | null): TreeNode | null {
    let first: TreeNode | null = null;
    let second: TreeNode | null = null;
    let prev: TreeNode | null = null;
    const stack: TreeNode[] = [];
    let node = root;
    while (stack.length > 0 || node !== null) {
        while (node !== null) {
            stack.push(node);
            node = node.left;
        }
        const top = stack.pop()!;
        if (prev !== null && prev.val > top.val) {
            if (first === null) first = prev;
            second = top;
        }
        prev = top;
        node = top.right;
    }
    const tmp = first!.val;
    first!.val = second!.val;
    second!.val = tmp;
    return root;
}
