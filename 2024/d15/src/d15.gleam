import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

type Point =
  #(Int, Int)

type Grid(a) =
  Dict(Point, a)

pub type Movement {
  Up
  Down
  Left
  Right
}

pub type Cell {
  Bot
  Box
  Floor
  Wall
}

fn print_grid(grid: Grid(Cell)) -> String {
  // Find grid boundaries
  let positions = dict.keys(grid)
  let assert Ok(max_row) =
    list.map(positions, fn(pos) { pos.0 })
    |> list.reduce(fn(a, b) { int.max(a, b) })
  let assert Ok(max_col) =
    list.map(positions, fn(pos) { pos.1 })
    |> list.reduce(fn(a, b) { int.max(a, b) })

  list.range(0, max_row)
  |> list.map(fn(row) {
    list.range(0, max_col)
    |> list.map(fn(col) {
      case dict.get(grid, #(row, col)) {
        Ok(Bot) -> "@"
        Ok(Box) -> "O"
        Ok(Floor) -> "."
        Ok(Wall) -> "#"
        _ -> " "
      }
    })
    |> string.join("")
  })
  |> string.join("\n")
}

fn char_to_cell(char: String) {
  case char {
    "@" -> Bot
    "O" -> Box
    "." -> Floor
    "#" -> Wall
    _ -> panic as { "Unknown cell type for char: '" <> char <> "'" }
  }
}

pub type Object {
  Object(position: Point)
}

fn get_bot_position_from_grid(grid: Grid(Cell)) -> Result(Object, Nil) {
  let pos =
    dict.filter(grid, fn(_key, val) { val == Bot })
    |> dict.to_list()
    |> list.first
  use pos <- result.try(pos)
  Ok(Object(pos.0))
}

fn char_to_movement(char: String) {
  case char {
    "^" -> Up
    ">" -> Right
    "v" -> Down
    "<" -> Left
    _ -> panic as { "Invalid movement character: '" <> char <> "'" }
  }
}

fn movement_to_relative_point(move: Movement) -> Point {
  case move {
    Up -> #(-1, 0)
    Right -> #(0, 1)
    Down -> #(1, 0)
    Left -> #(0, -1)
  }
}

fn parse_input(filename) {
  use input <- result.try(
    simplifile.read(filename) |> result.replace_error(Nil),
  )
  use #(map, movements) <- result.try(string.split_once(input, "\n\n"))
  let grid = parse_map(map)
  let moves = parse_movements(movements)
  Ok(#(grid, moves))
}

fn parse_map(map: String) -> Grid(Cell) {
  let map = map |> string.split("\n")
  use grid, row, row_idx <- list.index_fold(map, dict.new())
  use grid, col, col_idx <- list.index_fold(string.to_graphemes(row), grid)
  dict.insert(grid, #(row_idx, col_idx), char_to_cell(col))
}

fn parse_movements(movements: String) -> List(Movement) {
  string.trim(movements)
  |> string.replace("\n", "")
  |> string.to_graphemes()
  |> list.map(char_to_movement)
}

fn add_point(a: Point, b: Point) {
  #(a.0 + b.0, a.1 + b.1)
}

// fn get_moves(
//   grid: Grid(Cell),
//   current_pos: Point,
//   move_point: Point,
//   moves: List(Point),
// ) {
//   case dict.get(grid, current_pos) {
//     Ok(Wall) -> moves
//     Ok(Floor) -> [current_pos, ..moves]
//     Ok(Box) | Ok(Bot) ->
//       get_moves(grid, add_point(current_pos, move_point), move_point, [
//         current_pos,
//         ..moves
//       ])
//     _ -> panic as "Unexpected result found in position."
//   }
// }

fn get_push_chain(
  grid: Grid(Cell),
  start_pos: Point,
  move_point: Point,
  chain: List(Point),
) {
  let next_pos = add_point(start_pos, move_point)
  case dict.get(grid, next_pos) {
    // Invalid push, no chain
    Ok(Wall) -> []
    // Valid push, return chain
    Ok(Floor) -> [start_pos, ..chain]
    // Handle box
    Ok(Box) -> {
      get_push_chain(grid, next_pos, move_point, [start_pos, ..chain])
    }
    _ -> panic
  }
}

fn update_grid(grid: Grid(Cell), move: Movement) {
  // io.println(print_grid(grid))
  // io.println("---")
  // io.println("Attempt to move: " <> string.inspect(move))
  // io.println("---")
  let assert Ok(bot) = get_bot_position_from_grid(grid)
  let move_point = movement_to_relative_point(move)
  let next_pos = add_point(bot.position, move_point)

  case dict.get(grid, next_pos) {
    Ok(Wall) -> grid
    // Can't move into walls
    Ok(Floor) -> {
      // Simple move into unoccupied space 
      dict.insert(dict.insert(grid, bot.position, Floor), next_pos, Bot)
    }
    Ok(Box) -> {
      // Push box(es) if able.
      let push_chain = get_push_chain(grid, next_pos, move_point, [])
      case push_chain {
        [] -> grid
        // No chain, just return grid
        [_, ..] as box_chain -> {
          // A chain of at least one, move everything in chain
          // Move boxes forward
          let grid =
            list.fold(box_chain, grid, fn(grid, box_pos) {
              let box_next_pos = add_point(box_pos, move_point)
              dict.insert(dict.insert(grid, box_pos, Floor), box_next_pos, Box)
            })
          // Move bot forward
          dict.insert(dict.insert(grid, bot.position, Floor), next_pos, Bot)
        }
      }
    }
    _ -> grid
  }
}

fn score_grid(grid: Grid(Cell)) {
  // 100 * row + column
  let boxes = dict.filter(grid, fn(_key, value) { value == Box })
  use sum, position, _ <- dict.fold(boxes, 0)
  let #(row, col) = position
  sum + { 100 * row } + col
}

fn scale_grid(grid: Grid(Cell)) {
  todo
}

fn part_one(grid: Grid(Cell), moves: List(Movement)) {
  list.fold(moves, grid, fn(grid, move) { update_grid(grid, move) })
  |> score_grid
}

pub fn main() {
  let file = "input.in"
  let assert Ok(#(grid, moves)) = parse_input(file)
  part_one(grid, moves)
  |> io.debug
}
