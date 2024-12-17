import gleam/dict.{type Dict}
import gleam/io
import gleam/list
import gleam/option.{None, Some}
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

fn move_object(object: Object, move: Movement) {
  let new_pos = movement_to_relative_point(move)
  Object(#(object.position.0 + new_pos.0, object.position.1 + new_pos.1))
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

fn get_moves(
  grid: Grid(Cell),
  current_pos: Point,
  move_point: Point,
  moves: List(Point),
) {
  case dict.get(grid, current_pos) {
    Ok(Wall) -> moves
    Ok(Floor) -> [current_pos, ..moves]
    Ok(Box) | Ok(Bot) ->
      get_moves(grid, add_point(current_pos, move_point), move_point, [
        current_pos,
        ..moves
      ])
    _ -> panic as "Unexpected result found in position."
  }
}

fn update_grid(grid: Grid(Cell), move: Movement) {
  let assert Ok(bot) = get_bot_position_from_grid(grid)
  let moves =
    get_moves(grid, bot.position, movement_to_relative_point(move), [])
    |> list.reverse()
  let assert Ok(first_move) = list.first(moves)
  let move_windows = list.window(moves, 2)
  let grid =
    list.fold(move_windows, grid, fn(grid, moves) {
      dict.upsert(grid, result.unwrap(list.first(moves), #(0, 0)), fn(x) {
        todo
      })
    })
  let grid =
    dict.upsert(grid, first_move, fn(x) {
      case x {
        _ -> Floor
      }
    })
  grid
}

fn part_one(grid: Grid(Cell), moves: List(Movement)) {
  list.fold(moves, grid, fn(grid, move) { update_grid(grid, move) })
}

pub fn main() {
  let file = "sample.in"
  let assert Ok(#(grid, moves)) = parse_input(file)
  part_one(grid, moves)
  |> io.debug
}
