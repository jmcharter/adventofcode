import sys
from typing import Callable


def main():
    test_sample()
    args = sys.argv
    if len(args) < 2:
        print(f"Usage: {args[0]} <filename>")
        sys.exit(1)
    filename = args[1]
    with open(filename, "r") as f:
        file = f.read().strip()
        data = parse_input(file)
    part_one = process_input(data, predict_next)
    part_two = process_input(data, predict_prev)
    print(f"Part one: {part_one}")
    print(f"Part two: {part_two}")


def parse_input(file: str) -> list[list[int]]:
    return [[int(n) for n in line.split(" ")] for line in file.splitlines()]


def process_input(data: list[list[int]], predictor: Callable[[list], int]):
    hists = [get_history(item) for item in data]
    return sum([predictor(hist) for hist in hists])


def get_history(items: list[int]) -> list[list[int]]:
    results = [items]
    while True:
        last = results[-1]
        if all(e == 0 for e in last):
            return results
        results.append([r - l for l, r in zip(last, last[1:])])


def predict_prev(history: list) -> int:
    for l, r in zip(history[::-1], history[::-1][1:]):
        r.insert(0, r[0] - l[0])
    return history[0][0]


def predict_next(history: list) -> int:
    for l, r in zip(history[::-1], history[::-1][1:]):
        r.append(l[-1] + r[-1])
    return history[0][-1]


def test_sample():
    sample = """0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"""
    data = parse_input(sample)
    result1 = process_input(data, predict_next)
    expect1 = 114
    result2 = process_input(data, predict_prev)
    expect2 = 2
    assert result1 == expect1, f"Expected {expect1}, got {result1}"
    assert result2 == expect2, f"Expected {expect2}, got {result2}"


if __name__ == "__main__":
    main()
