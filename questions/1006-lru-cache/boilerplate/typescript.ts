// 1006. LRU Cache (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class LRUCache {
  constructor(capacity: number) {

  }

  get(key: number): number {

  }

  put(key: number, value: number): void {

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
