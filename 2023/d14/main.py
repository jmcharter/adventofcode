import sys

Platform = list[str]
PlatformMap = dict[tuple[int, int], int]


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


def platform_to_map(platform: Platform) -> PlatformMap:
    platform_map = {}
    for y, line in enumerate(platform):
        for x, ch in enumerate(line):
            if ch == "O":
                platform_map[(x, y)] = 1
            elif ch == "#":
                platform_map[(x, y)] = 2
            else:
                platform_map[(x, y)] = 0
    return platform_map


def map_to_platform(map: dict) -> Platform:
    s = ""
    digit_to_char = {0: ".", 1: "O", 2: "#"}
    for k, v in map.items():
        if k[0] == 0:
            s = s + f"\n{digit_to_char[v]}"
        else:
            s = s + digit_to_char[v]
    return s.strip().splitlines()


def tilt_platform_map(map: PlatformMap) -> PlatformMap:
    for coord, entity in map.items():
        if entity != 1:
            continue
        highest_coord = get_furthest_location(map, coord)
        map[coord] = 0
        map[highest_coord] = 1
    return map


def sum_platform(platform: Platform) -> int:
    height = len(platform)
    s = 0
    for y, row in enumerate(platform):
        for x, ch in enumerate(row):
            if ch == "O":
                s += height - y
    return s


def get_furthest_location(map: PlatformMap, rock: list):
    x, y = rock
    new_point = y
    for loc in range(y - 1, -1, -1):
        if map[(x, loc)] == 0:
            new_point = loc
        else:
            return (x, new_point)
    return (x, new_point)


def part_one(platform: Platform):
    return sum_platform(map_to_platform(tilt_platform_map(platform_to_map(platform))))


def part_two(platform: Platform):
    cycle_count = 0
    found = [platform]
    total_cycles = 1_000_000_000
    loop_start_idx = 0
    while cycle_count < total_cycles:
        for i in range(4):
            platform_map = platform_to_map(platform)
            tilted_platform_map = tilt_platform_map(platform_map)
            tilted_platform = map_to_platform(tilted_platform_map)
            rotated_platform = rotate_platform(tilted_platform)
            platform = rotated_platform
        if platform in found:
            loop_start_idx = found.index(platform)
            loop_len = cycle_count + 1 - loop_start_idx
            break
        found.append(platform)
        cycle_count += 1

    result = sum_platform(
        found[(total_cycles - loop_start_idx) % loop_len + loop_start_idx]
    )
    return result


def main():
    args = sys.argv
    if len(args) < 2:
        print(f"Usage: {args[0]} <filename>")
        sys.exit(1)
    filename = args[1]
    with open(filename, "r") as f:
        data = f.read().strip().splitlines()

    p1 = part_one(data)
    p2 = part_two(data)
    print(f"Part one: {p1}")
    print(f"Part two: {p2}")


if __name__ == "__main__":
    main()
