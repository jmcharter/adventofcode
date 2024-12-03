import gleam/int
import gleam/io
import gleam/list
import gleam/regex
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  let assert Ok(data) = simplifile.read("input.in")
  part_one(data) |> io.debug
  part_two(data) |> io.debug
}

fn part_one(data) {
  let assert Ok(multiplication_pattern) =
    regex.from_string("mul\\(\\d{1,3},\\d{1,3}\\)")
  let assert Ok(digit_pattern) = regex.from_string("\\d{1,3},\\d{1,3}")
  let multiplications =
    regex.scan(multiplication_pattern, data)
    |> list.flat_map(fn(reg) {
      regex.scan(digit_pattern, reg.content)
      |> list.map(fn(digits) {
        digits.content
        |> string.split(",")
        |> list.map(fn(x) { x |> int.parse |> result.unwrap(0) })
        |> list.reduce(fn(a, b) { a * b })
        |> result.unwrap(0)
      })
    })
    |> list.reduce(fn(a, b) { a + b })
    |> result.unwrap(0)
}

fn part_two(data) {
  let data = "do()" <> string.replace(data, "\n", "") <> "don't()"
  let assert Ok(pattern) = regex.from_string("do\\(\\).*?don't\\(\\)")
  regex.scan(pattern, data)
  |> list.map(fn(input) { input.content |> part_one })
  |> list.reduce(fn(a, b) { a + b })
}
