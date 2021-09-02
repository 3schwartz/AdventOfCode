card_key, door_key = [int(key) for key in "5099500-7648211".split("-")]

divider = 20201227


def get_loop_size(key):
    loop, subject = 0, 1

    while subject != key:
        subject = subject * 7 % divider
        loop += 1

    return loop


def get_encryption(loop, key):
    subject = 1
    for i in range(loop):
        subject = subject * key % divider
    return subject


card_loop = get_loop_size(card_key)
door_loop = get_loop_size(door_key)

print(get_encryption(card_loop, door_key))
print(get_encryption(door_loop, card_key))
