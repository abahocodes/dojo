function sortedListToBst(head) {
  let n = 0;
  for (let node = head; node !== null; node = node.next) n++;

  let cur = head;
  function build(lo, hi) {
    if (lo > hi) return null;
    const mid = (lo + hi + 1) >> 1;
    const left = build(lo, mid - 1);
    const root = new TreeNode(cur.val, left, null);
    cur = cur.next;
    root.right = build(mid + 1, hi);
    return root;
  }
  return build(0, n - 1);
}
