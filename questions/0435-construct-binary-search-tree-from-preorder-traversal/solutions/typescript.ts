function bstFromPreorder(preorder: number[]): TreeNode | null {
    const root = new TreeNode(preorder[0]);
    const stack: TreeNode[] = [root];
    for (let i = 1; i < preorder.length; i++) {
        const v = preorder[i];
        const node = new TreeNode(v);
        if (v < stack[stack.length - 1].val) {
            stack[stack.length - 1].left = node;
        } else {
            let parent = stack.pop()!;
            while (stack.length > 0 && stack[stack.length - 1].val < v) parent = stack.pop()!;
            parent.right = node;
        }
        stack.push(node);
    }
    return root;
}
