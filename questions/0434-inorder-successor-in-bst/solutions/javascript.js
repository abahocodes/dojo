function inorderSuccessor(root, p) {
  let answer = -1;
  let node = root;
  while (node !== null) {
    if (node.val > p) {
      answer = node.val;
      node = node.left;
    } else {
      node = node.right;
    }
  }
  return answer;
}
