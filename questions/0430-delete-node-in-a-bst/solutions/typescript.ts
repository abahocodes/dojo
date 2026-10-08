function deleteNode(root: TreeNode | null, key: number): TreeNode | null {
    let parent: TreeNode | null = null;
    let node = root;
    while (node !== null && node.val !== key) {
        parent = node;
        node = key < node.val ? node.left : node.right;
    }
    if (node === null) return root;

    if (node.left !== null && node.right !== null) {
        let succParent: TreeNode = node;
        let succ: TreeNode = node.right;
        while (succ.left !== null) {
            succParent = succ;
            succ = succ.left;
        }
        node.val = succ.val;
        if (succParent === node) succParent.right = succ.right;
        else succParent.left = succ.right;
        return root;
    }

    const child = node.left !== null ? node.left : node.right;
    if (parent === null) return child;
    if (parent.left === node) parent.left = child;
    else parent.right = child;
    return root;
}
