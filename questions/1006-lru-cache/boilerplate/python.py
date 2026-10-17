# 1006. LRU Cache (medium)
# /show to reread the problem · /test runs visible tests · /submit when done

class LRUCache:
    def __init__(self, capacity: int):
        pass

    def get(self, key: int) -> int:
        pass

    def put(self, key: int, value: int) -> None:
        pass


# dojo calls this to run the tests; you don't need to change it.
def lru_cache(capacity: int, ops: list[str], args: list[list[int]]) -> list[int]:
    cache = LRUCache(capacity)
    out = []
    for op, a in zip(ops, args):
        if op == "put":
            cache.put(a[0], a[1])
        else:
            out.append(cache.get(a[0]))
    return out
