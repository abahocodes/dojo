// 1006. LRU Cache (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class LRUCache {
  /**
   * @param {number} capacity
   */
  constructor(capacity) {

  }

  /**
   * @param {number} key
   * @return {number}
   */
  get(key) {

  }

  /**
   * @param {number} key
   * @param {number} value
   * @return {void}
   */
  put(key, value) {

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
