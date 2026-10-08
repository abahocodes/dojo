// pre-order with explicit null markers; the leading comma on every token
// keeps "2" from matching inside "12"
function encode(root: TreeNode | null): string {
    const parts: string[] = [];
    const stack: (TreeNode | null)[] = [root];
    while (stack.length > 0) {
        const node = stack.pop()!;
        if (node === null) {
            parts.push(",#");
        } else {
            parts.push("," + node.val);
            stack.push(node.right);
            stack.push(node.left);
        }
    }
    return parts.join("");
}

function isSubtree(root: TreeNode | null, subRoot: TreeNode | null): boolean {
    return encode(root).includes(encode(subRoot));
}
