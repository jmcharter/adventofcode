defmodule Day02 do
  defp part1(reports) do
    reports
    |> Enum.map(fn report ->
      levels =
        report
        |> String.split()
        |> Enum.map(&String.to_integer/1)

      cond do
        sequence_is_safe?(levels) ->
          :safe

        true ->
          :unsafe
      end
    end)
    |> Enum.count(fn x -> x == :safe end)
  end

  defp part2(reports) do
    reports
    |> Enum.map(fn report ->
      levels =
        report
        |> String.split()
        |> Enum.map(&String.to_integer/1)

      sequences =
        0..(length(levels) - 1)
        |> Enum.map(fn i ->
          List.delete_at(levels, i)
        end)

      cond do
        sequence_is_safe?(levels) ->
          :safe

        Enum.any?(sequences, &sequence_is_safe?/1) ->
          :safe

        true ->
          :unsafe
      end
    end)
    |> Enum.count(fn x -> x == :safe end)
  end

  defp all_gaps_within_max_diff?(numbers) do
    numbers
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.all?(fn [a, b] -> abs(b - a) <= 3 end)
  end

  defp is_strictly_increasing?(numbers) do
    numbers
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.all?(fn [a, b] -> a < b end)
  end

  defp is_strictly_decreasing?(numbers) do
    numbers
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.all?(fn [a, b] -> a > b end)
  end

  defp sequence_is_safe?(numbers) do
    (is_strictly_increasing?(numbers) or
       is_strictly_decreasing?(numbers)) and all_gaps_within_max_diff?(numbers)
  end

  def run(data) do
    reports = data |> String.split("\n", trim: true)
    p1 = part1(reports)
    p2 = part2(reports)
    IO.puts(p1)
    IO.puts(p2)
  end
end

data = File.read!("input.in")
Day02.run(data)
