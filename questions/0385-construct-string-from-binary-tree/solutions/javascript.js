function tree2str(root) {
  const parts = [];
  const stack = [root];
  while (stack.length > 0) {
    const item = stack.pop();
    if (typeof item === "string") {
      parts.push(item);
      continue;
    }
    parts.push(String(item.val));
    if (item.right) stack.push(")", item.right, "(");
    if (item.left) stack.push(")", item.left, "(");
    else if (item.right) stack.push("()");
  }
  return parts.join("");
}
