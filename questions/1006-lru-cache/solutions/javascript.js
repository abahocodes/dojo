class LRUCache {
  constructor(capacity) {
    this.capacity = capacity;
    // A Map iterates in insertion order: the first key is the least recently used.
    this.map = new Map();
  }

  get(key) {
    if (!this.map.has(key)) return -1;
    const value = this.map.get(key);
    this.map.delete(key);
    this.map.set(key, value);
    return value;
  }

  put(key, value) {
    this.map.delete(key);
    this.map.set(key, value);
    if (this.map.size > this.capacity) {
      this.map.delete(this.map.keys().next().value);
    }
  }
}

// dojo calls this to run the tests; you don't need to change it.
function lruCache(capacity, ops, args) {
  const cache = new LRUCache(capacity);
  const out = [];
  for (let i = 0; i < ops.length; i++) {
    if (ops[i] === "put") cache.put(args[i][0], args[i][1]);
    else out.push(cache.get(args[i][0]));
  }
  return out;
}
