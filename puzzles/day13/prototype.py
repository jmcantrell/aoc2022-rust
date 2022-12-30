import sys
import json
from itertools import zip_longest
from functools import cmp_to_key


def cmp(a, b):
    if a is None:
        return -1
    elif b is None:
        return 1
    elif isinstance(a, int) and isinstance(b, int):
        if a == b:
            return 0
        elif a < b:
            return -1
        else:
            return 1
    elif isinstance(a, list) and isinstance(b, list):
        for x, y in zip_longest(a, b):
            if (res := cmp(x, y)) != 0:
                return res
        return 0
    elif isinstance(a, int) and isinstance(b, list):
        return cmp([a], b)
    elif isinstance(a, list) and isinstance(b, int):
        return cmp(a, [b])
    else:
        raise Exception("input seems invalid")


def parse(file):
    with open(file) as f:
        for i, chunk in enumerate(f.read().strip().split("\n\n")):
            yield [json.loads(line) for line in chunk.split("\n")]


def part1(file):
    sum_of_nums = 0

    for i, pair in enumerate(parse(file)):
        if cmp(*pair) < 0:
            sum_of_nums += i + 1

    return sum_of_nums


def part2(file):
    pairs = []
    for a, b in parse(file):
        pairs.append(a)
        pairs.append(b)

    divider_a = [[2]]
    divider_b = [[6]]

    pairs.append(divider_a)
    pairs.append(divider_b)

    decoder_key = 1

    for i, pair in enumerate(sorted(pairs, key=cmp_to_key(cmp))):
        if pair is divider_a or pair is divider_b:
            decoder_key *= i + 1

    return decoder_key


def main(file):
    print(f"Solution to part 1: {part1(file)}")
    print(f"Solution to part 2: {part2(file)}")


if __name__ == "__main__":
    main(sys.argv[1])
