function encode(root) {
  const parts = [];
  const stack = [root];
  while (stack.length > 0) {
    const node = stack.pop();
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

function isSubtree(root, subRoot) {
  return encode(root).includes(encode(subRoot));
}
