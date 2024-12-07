import gleam/dict
import gleam/io
import gleam/list
import gleam/option.{None, Some}
import gleam/result
import gleam/set.{type Set}
import gleam/string
import simplifile

pub type Point =
  #(Int, Int)

pub type Grid(a) =
  dict.Dict(Point, a)

pub type Direction {
  North
  East
  South
  West
}

pub type Loops {
  DoesLoop
  DoesNotLoop
}

pub type Guard {
  Guard(position: Point, direction: Direction)
}

fn get_guard(grid: Grid(String)) -> Guard {
  let pos = dict.filter(grid, fn(_pos, char) { char == "^" })
  let assert Ok(pos) = case dict.size(pos) {
    1 -> list.first(dict.keys(pos))
    0 -> panic as "No guard found in input!"
    _ -> panic as "More than one guard found in input!"
  }
  Guard(pos, North)
}

fn move_guard(guard: Guard) -> Guard {
  let new_pos = case guard.direction {
    North -> #(-1, 0)
    East -> #(0, 1)
    South -> #(1, 0)
    West -> #(0, -1)
  }
  Guard(
    #(guard.position.0 + new_pos.0, guard.position.1 + new_pos.1),
    guard.direction,
  )
}

fn turn_guard(guard: Guard) -> Guard {
  let new_dir = case guard.direction {
    North -> East
    East -> South
    South -> West
    West -> North
  }
  Guard(guard.position, new_dir)
}

fn get_obstacles(grid: Grid(String)) -> List(Point) {
  dict.filter(grid, fn(_pos, char) { char == "#" })
  |> dict.keys()
}

fn recurse_grid(
  grid: Grid(String),
  guard: Guard,
  obstacles: List(#(Int, Int)),
  visited: Set(#(#(Int, Int), Direction)),
) -> #(Set(#(#(Int, Int), Direction)), Loops) {
  let new_guard = move_guard(guard)
  let position = new_guard.position
  let dir = new_guard.direction
  case dict.has_key(grid, position) {
    False -> #(visited, DoesNotLoop)
    True -> {
      case set.contains(visited, #(position, dir)) {
        True -> {
          #(visited, DoesLoop)
        }
        False -> {
          case list.contains(obstacles, position) {
            True -> recurse_grid(grid, turn_guard(guard), obstacles, visited)
            False ->
              recurse_grid(
                grid,
                new_guard,
                obstacles,
                set.insert(visited, #(position, dir)),
              )
          }
        }
      }
    }
  }
}

fn get_grid_input(filename: String) -> Grid(String) {
  let lines =
    filename
    |> simplifile.read()
    |> result.unwrap("")
    |> string.trim()
    |> string.split("\n")
  use grid, row, row_idx <- list.index_fold(lines, dict.new())
  use grid, col, col_idx <- list.index_fold(string.to_graphemes(row), grid)
  dict.insert(grid, #(row_idx, col_idx), col)
}

fn part_one(
  grid: Grid(String),
) -> #(#(Set(#(#(Int, Int), Direction)), Loops), Int) {
  let guard = get_guard(grid)
  let obstacles = get_obstacles(grid)
  let visited = set.new() |> set.insert(#(guard.position, guard.direction))
  let visited = recurse_grid(grid, guard, obstacles, visited)
  let visited_without_dir =
    set.fold(visited.0, set.new(), fn(acc, x) { set.insert(acc, x.0) })
  #(visited, visited_without_dir |> set.size())
}

fn check_loop(grid: Grid(String), blocker: Point) -> Loops {
  let blocked_grid =
    dict.upsert(grid, blocker, fn(x) {
      case x {
        Some("^") -> "^"
        Some(_) -> "#"
        None -> "#"
      }
    })
  let visited = part_one(blocked_grid).0
  visited.1
}

fn part_two(grid: Grid(String), visited: Set(#(#(Int, Int), Direction))) {
  let visited =
    set.fold(visited, set.new(), fn(acc, x) { set.insert(acc, x.0) })
  use counter, position <- set.fold(visited, 0)
  case check_loop(grid, position) {
    DoesLoop -> counter + 1
    DoesNotLoop -> counter
  }
}

pub fn main() {
  let input = "input.in"
  let p1 = input |> get_grid_input() |> part_one
  let visited = p1.0.0
  io.debug(p1.1)
  input |> get_grid_input |> part_two(visited) |> io.debug()
}
