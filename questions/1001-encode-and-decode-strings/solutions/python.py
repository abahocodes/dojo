def encode(strs: list[str]) -> str:
    # Each string becomes "<length>#<string>"; the length says where it ends.
    return "".join(f"{len(s)}#{s}" for s in strs)


def decode(s: str) -> list[str]:
    result = []
    i = 0
    while i < len(s):
        j = s.index("#", i)
        length = int(s[i:j])
        result.append(s[j + 1 : j + 1 + length])
        i = j + 1 + length
    return result


# dojo calls this to run the tests; you don't need to change it.
def encode_and_decode(strs: list[str]) -> list[str]:
    encoded = encode(list(strs))
    other = encode(list(reversed(strs)) + ["dojo"])
    if not isinstance(encoded, str) or not isinstance(other, str):
        return ["<encode must return one string>"]
    return decode(encoded)
