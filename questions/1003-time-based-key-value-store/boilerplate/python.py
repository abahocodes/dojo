# 1003. Time Based Key-Value Store (medium)
# /show to reread the problem · /test runs visible tests · /submit when done

class TimeMap:
    def __init__(self):
        pass

    def set(self, key: str, value: str, timestamp: int) -> None:
        pass

    def get(self, key: str, timestamp: int) -> str:
        pass


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
