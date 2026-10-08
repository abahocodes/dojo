def get_decimal_value(head):
    value = 0
    while head:
        value = value * 2 + head.val
        head = head.next
    return value
