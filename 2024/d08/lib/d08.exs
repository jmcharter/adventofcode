defmodule D08 do
  defp parse_input(filename) do
    File.read!(filename)
    |> String.split("\n", trim: true)
    |> Enum.with_index()
    |> Enum.reduce(Map.new(), fn {line, line_idx}, grid ->
      line
      |> String.graphemes()
      |> Enum.with_index()
      |> Enum.reduce(grid, fn {col, col_idx}, grid ->
        grid
        |> Map.put({line_idx, col_idx}, col)
      end)
    end)
  end

  def pairs([]) do
    []
  end

  def pairs([_]) do
    []
  end

  def pairs([head | tail]) do
    head_pairs = Enum.map(tail, fn x -> {head, x} end)
    head_pairs ++ pairs(tail)
  end

  def extrapolate_points(point_pairs) do
    Enum.flat_map(point_pairs, fn {{ax, ay}, {bx, by}} ->
      {dx, dy} =
        {bx - ax, by - ay}

      [{ax - dx, ay - dy}, {bx + dx, by + dy}]
    end)
  end

  def extrapolate_points(point_pairs, {max_x, max_y}) do
    Enum.flat_map(point_pairs, fn {{ax, ay}, {bx, by}} ->
      {dx, dy} =
        {bx - ax, by - ay}

      outer =
        Stream.iterate({bx, by}, fn {x, y} -> {x - dx, y - dy} end)
        |> Enum.take_while(fn {x, y} -> x >= 0 and x <= max_x and y >= 0 and y <= max_y end)

      inner =
        Stream.iterate({ax, ay}, fn {x, y} -> {x + dx, y + dy} end)
        |> Enum.take_while(fn {x, y} -> x >= 0 and x <= max_x and y >= 0 and y <= max_y end)

      outer ++ inner
    end)
  end

  def process(input, part) do
    grid = parse_input(input)

    frequencies =
      MapSet.new(Map.values(grid))
      |> MapSet.delete(".")

    Enum.flat_map(frequencies, fn f ->
      points =
        Map.filter(grid, fn {_key, value} ->
          value == f
        end)

      freq_pairs = pairs(Map.keys(points))
      max_x = grid |> Map.keys() |> Enum.map(&elem(&1, 0)) |> Enum.max()
      max_y = grid |> Map.keys() |> Enum.map(&elem(&1, 1)) |> Enum.max()

      case part do
        :part_1 ->
          extrapolate_points(freq_pairs)

        :part_2 ->
          extrapolate_points(freq_pairs, {max_x, max_y})
      end
    end)
    |> MapSet.new()
    |> MapSet.filter(&Map.has_key?(grid, &1))
    |> MapSet.size()
  end

  def run do
    process("input.in", :part_1)
    |> IO.inspect()

    process("input.in", :part_2)
    |> IO.inspect()
  end
end

D08.run()
