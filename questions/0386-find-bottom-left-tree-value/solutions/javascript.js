function findBottomLeftValue(root) {
  const queue = [root];
  let node = root;
  for (let head = 0; head < queue.length; head++) {
    node = queue[head];
    if (node.right) queue.push(node.right);
    if (node.left) queue.push(node.left);
  }
  return node.val;
}
