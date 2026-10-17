class TimeMap {
  // key -> { times, values }; set timestamps only increase, so both stay sorted by time.
  private history = new Map<string, { times: number[]; values: string[] }>();

  set(key: string, value: string, timestamp: number): void {
    let h = this.history.get(key);
    if (!h) {
      h = { times: [], values: [] };
      this.history.set(key, h);
    }
    h.times.push(timestamp);
    h.values.push(value);
  }

  get(key: string, timestamp: number): string {
    const h = this.history.get(key);
    if (!h) return "";
    // Find the first write after `timestamp`; the one before it is the answer.
    let lo = 0;
    let hi = h.times.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (h.times[mid] <= timestamp) lo = mid + 1;
      else hi = mid;
    }
    return lo > 0 ? h.values[lo - 1] : "";
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
