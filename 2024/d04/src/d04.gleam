import gleam/dict
import gleam/io
import gleam/list
import gleam/string
import simplifile

pub type Point =
  #(Int, Int)

pub type Grid(key) {
  Grid(dict.Dict(Point, key))
}

pub fn main() {
  let data = parse_data("sample.in")
  part_one(data) |> io.debug()
}

fn part_one(data) {
  data
}

fn parse_data(data) {
  let assert Ok(data) = simplifile.read(data)
  string.trim(data)
  |> string.split("\n")
  |> list.index_fold(dict.new(), fn(grid, line, line_idx) {
    use grid, char, column_idx <- list.index_fold(
      string.to_graphemes(line),
      grid,
    )
    dict.insert(grid, #(line_idx, column_idx), char)
  })
}
