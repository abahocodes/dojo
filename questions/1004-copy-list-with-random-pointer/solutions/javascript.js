class Node {
  constructor(val, next = null, random = null) {
    this.val = val;
    this.next = next;
    this.random = random;
  }
}

function copyRandomList(head) {
  // First pass: one new node per original node.
  const copies = new Map();
  for (let node = head; node !== null; node = node.next) {
    copies.set(node, new Node(node.val));
  }
  // Second pass: wire next and random through the map.
  for (let node = head; node !== null; node = node.next) {
    const copy = copies.get(node);
    copy.next = node.next === null ? null : copies.get(node.next);
    copy.random = node.random === null ? null : copies.get(node.random);
  }
  return head === null ? null : copies.get(head);
}

// dojo calls this to run the tests; you don't need to change it.
function runCopyRandomList(nodes) {
  const originals = nodes.map(([val]) => new Node(val));
  const nextOf = (i) => (i + 1 < originals.length ? originals[i + 1] : null);
  const randomOf = (r) => (r >= 0 ? originals[r] : null);
  nodes.forEach(([, r], i) => {
    originals[i].next = nextOf(i);
    originals[i].random = randomOf(r);
  });
  const copy = copyRandomList(originals.length > 0 ? originals[0] : null);

  // The original list must be left as it was.
  for (let i = 0; i < nodes.length; i++) {
    const node = originals[i];
    if (node.val !== nodes[i][0] || node.next !== nextOf(i) || node.random !== randomOf(nodes[i][1])) {
      return [[-1]];
    }
  }
  // Walk the copy: every node must be new, and the list must end.
  const originalSet = new Set(originals);
  const index = new Map();
  const order = [];
  for (let node = copy; node != null; node = node.next) {
    if (originalSet.has(node) || index.has(node)) return [[-1]];
    index.set(node, order.length);
    order.push(node);
  }
  const result = [];
  for (const node of order) {
    if (node.random == null) result.push([node.val, -1]);
    else if (index.has(node.random)) result.push([node.val, index.get(node.random)]);
    else return [[-1]];
  }
  return result;
}
