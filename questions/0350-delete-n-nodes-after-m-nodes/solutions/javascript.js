function deleteNodes(head, m, n) {
  let cur = head;
  while (cur !== null) {
    for (let i = 0; i < m - 1; i++) {
      if (cur.next === null) return head;
      cur = cur.next;
    }
    let skip = cur.next;
    for (let i = 0; i < n && skip !== null; i++) {
      skip = skip.next;
    }
    cur.next = skip;
    cur = skip;
  }
  return head;
}
