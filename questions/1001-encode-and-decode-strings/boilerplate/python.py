# 1001. Encode and Decode Strings (medium)
# /show to reread the problem · /test runs visible tests · /submit when done

def encode(strs: list[str]) -> str:
    pass


def decode(s: str) -> list[str]:
    pass


# dojo calls this to run the tests; you don't need to change it.
def encode_and_decode(strs: list[str]) -> list[str]:
    encoded = encode(list(strs))
    other = encode(list(reversed(strs)) + ["dojo"])
    if not isinstance(encoded, str) or not isinstance(other, str):
        return ["<encode must return one string>"]
    return decode(encoded)
