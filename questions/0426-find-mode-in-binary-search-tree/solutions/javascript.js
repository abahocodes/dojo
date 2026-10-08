function findMode(root) {
  let modes = [];
  let best = 0;
  let count = 0;
  let prev = null;
  const stack = [];
  let node = root;
  while (stack.length > 0 || node) {
    while (node) {
      stack.push(node);
      node = node.left;
    }
    node = stack.pop();
    count = node.val === prev ? count + 1 : 1;
    prev = node.val;
    if (count > best) {
      best = count;
      modes = [node.val];
    } else if (count === best) {
      modes.push(node.val);
    }
    node = node.right;
  }
  return modes;
}
