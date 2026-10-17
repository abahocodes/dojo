// 1003. Time Based Key-Value Store (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class TimeMap {
  constructor() {

  }

  set(key: string, value: string, timestamp: number): void {

  }

  get(key: string, timestamp: number): string {

  }
}

// dojo calls this to run the tests; you don't need to change it.
function timeMap(ops: string[], keys: string[], values: string[], timestamps: number[]): string[] {
  const store = new TimeMap();
  const out: string[] = [];
  for (let i = 0; i < ops.length; i++) {
    if (ops[i] === "set") store.set(keys[i], values[i], timestamps[i]);
    else out.push(store.get(keys[i], timestamps[i]));
  }
  return out;
}
