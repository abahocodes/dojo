class DNode {
  prev: DNode | null = null;
  next: DNode | null = null;
  key: number;
  value: number;

  constructor(key: number, value: number) {
    this.key = key;
    this.value = value;
  }
}

class LRUCache {
  private nodes = new Map<number, DNode>();
  // Dummy ends: head.next is the most recently used, tail.prev the least.
  private head = new DNode(0, 0);
  private tail = new DNode(0, 0);
  private capacity: number;

  constructor(capacity: number) {
    this.capacity = capacity;
    this.head.next = this.tail;
    this.tail.prev = this.head;
  }

  private unlink(node: DNode): void {
    node.prev!.next = node.next;
    node.next!.prev = node.prev;
  }

  private pushFront(node: DNode): void {
    node.prev = this.head;
    node.next = this.head.next;
    this.head.next!.prev = node;
    this.head.next = node;
  }

  get(key: number): number {
    const node = this.nodes.get(key);
    if (!node) return -1;
    this.unlink(node);
    this.pushFront(node);
    return node.value;
  }

  put(key: number, value: number): void {
    let node = this.nodes.get(key);
    if (node) {
      node.value = value;
      this.unlink(node);
    } else {
      node = new DNode(key, value);
      this.nodes.set(key, node);
      if (this.nodes.size > this.capacity) {
        const lru = this.tail.prev!;
        this.unlink(lru);
        this.nodes.delete(lru.key);
      }
    }
    this.pushFront(node);
  }
}

// dojo calls this to run the tests; you don't need to change it.
function lruCache(capacity: number, ops: string[], args: number[][]): number[] {
  const cache = new LRUCache(capacity);
  const out: number[] = [];
  for (let i = 0; i < ops.length; i++) {
    if (ops[i] === "put") cache.put(args[i][0], args[i][1]);
    else out.push(cache.get(args[i][0]));
  }
  return out;
}
