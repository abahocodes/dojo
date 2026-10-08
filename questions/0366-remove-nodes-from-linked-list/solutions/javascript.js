function removeNodes(head) {
  const stack = [];
  for (let node = head; node !== null; node = node.next) {
    while (stack.length > 0 && stack[stack.length - 1].val < node.val) stack.pop();
    stack.push(node);
  }
  for (let i = 0; i + 1 < stack.length; i++) stack[i].next = stack[i + 1];
  stack[stack.length - 1].next = null;
  return stack[0];
}
