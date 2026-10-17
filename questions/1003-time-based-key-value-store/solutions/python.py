from bisect import bisect_right


class TimeMap:
    def __init__(self):
        # key -> (timestamps, values); set timestamps only increase, so both stay sorted by time.
        self.history: dict[str, tuple[list[int], list[str]]] = {}

    def set(self, key: str, value: str, timestamp: int) -> None:
        times, values = self.history.setdefault(key, ([], []))
        times.append(timestamp)
        values.append(value)

    def get(self, key: str, timestamp: int) -> str:
        if key not in self.history:
            return ""
        times, values = self.history[key]
        i = bisect_right(times, timestamp)  # first write after `timestamp`
        return values[i - 1] if i > 0 else ""


# dojo calls this to run the tests; you don't need to change it.
def time_map(ops: list[str], keys: list[str], values: list[str], timestamps: list[int]) -> list[str]:
    store = TimeMap()
    out = []
    for op, key, value, timestamp in zip(ops, keys, values, timestamps):
        if op == "set":
            store.set(key, value, timestamp)
        else:
            out.append(store.get(key, timestamp))
    return out
