function removeNodes(head: ListNode | null): ListNode | null {
  const stack: ListNode[] = [];
  for (let node: ListNode | null = head; node !== null; node = node.next) {
    while (stack.length > 0 && stack[stack.length - 1].val < node.val) stack.pop();
    stack.push(node);
  }
  if (stack.length === 0) return null;
  for (let i = 0; i + 1 < stack.length; i++) stack[i].next = stack[i + 1];
  stack[stack.length - 1].next = null;
  return stack[0];
}
