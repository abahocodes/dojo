function swapNodes(head, k) {
  let first = head;
  for (let i = 1; i < k; i++) first = first.next;
  let runner = first;
  let second = head;
  while (runner.next !== null) {
    runner = runner.next;
    second = second.next;
  }
  const t = first.val;
  first.val = second.val;
  second.val = t;
  return head;
}
