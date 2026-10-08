function deserialize(data: string): TreeNode | null {
  const tokens = data.split(",");
  if (tokens[0] === "#") return null;
  const root = new TreeNode(parseInt(tokens[0], 10));
  const stack: TreeNode[] = [root];
  const leftDone: boolean[] = [false];
  for (let i = 1; i < tokens.length; i++) {
    const tok = tokens[i];
    const child = tok === "#" ? null : new TreeNode(parseInt(tok, 10));
    const top = stack.length - 1;
    if (!leftDone[top]) {
      stack[top].left = child;
      leftDone[top] = true;
    } else {
      stack[top].right = child;
      stack.pop();
      leftDone.pop();
    }
    if (child !== null) {
      stack.push(child);
      leftDone.push(false);
    }
  }
  return root;
}
