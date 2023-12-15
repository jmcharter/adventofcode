import sys

Platform = list[str]


def main():
    args = sys.argv
    if len(args) < 2:
        print(f"Usage: {args[0]} <filename>")
        sys.exit(1)
    filename = args[1]
    with open(filename, 'r') as f:
        data = f.read().strip().splitlines()

    p1 = part_one(data)
    print(f"Part one: {p1}")


def print_platform(platform: Platform):
    for x in platform:
        print(x)
    print("---")


def rotate_platform(platform: Platform) -> Platform:
    return ["".join(list(z)) for z in zip(*platform[::-1])]


def multi_rotate(rotations: int, platform: Platform) -> Platform:
    res = platform
    for i in range(rotations):
        res = rotate_platform(res)
    return res


def map_to_platform(map) -> Platform:
    s = ""
    digit_to_char = {0: '.', 1: 'O', 2: '#'}
    for k, v in map.items():
        if k[0] == 0:
            s = s+f"\n{digit_to_char[v]}"
        else:
            s = s+digit_to_char[v]
    return s.strip().splitlines()


def part_one(platform: Platform):
    height = len(platform)
    count = 0
    found = []
    results = {}
    recurring = False
    reps = 1_000_000_000
    while count < reps:
        if recurring and count % (reps % len(found)) == 0:
            break
        if count % 4 == 0:
            if platform in found:
                recurring = True
            found.append(platform)
        platform_map = {}
        for y, line in enumerate(platform):
            for x, ch in enumerate(line):
                if ch == 'O':
                    platform_map[(x, y)] = 1
                elif ch == '#':
                    platform_map[(x, y)] = 2
                else:
                    platform_map[(x, y)] = 0
        for coord, entity in platform_map.items():
            if entity != 1:
                continue
            highest_coord = get_furthest_location(
                platform_map, coord)
            platform_map[coord] = 0
            platform_map[highest_coord] = 1
        platform = map_to_platform(platform_map)
        print_platform(multi_rotate(4-count % 4, platform))
        count += 1
        s = sum([1 * (height-k[1])
                 for (k, v) in platform_map.items() if v == 1])
        s2 = 0
        for y, row in enumerate(multi_rotate(4-count % 4, platform)):
            for x, ch in enumerate(row):
                if ch == 'O':
                    s2 += 1*height-y
        platform = rotate_platform(platform)
        print(s)
        print(s2)
        results[count] = s
    print(count)
    print(results)
    print(s)


def get_furthest_location(map: dict, rock: list):
    x, y = rock
    new_point = y
    for loc in range(y-1, -1, -1):
        if map[(x, loc)] == 0:
            new_point = loc
        else:
            return (x, new_point)
    return (x, new_point)


if __name__ == "__main__":
    main()
