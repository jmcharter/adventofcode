part_one =
  File.read!("input.in")
  |> String.split("\n", trim: true)
  |> Enum.map(fn line ->
    line
    |> String.split()
    |> Enum.map(&String.to_integer/1)
  end)
  |> Enum.reduce({[], []}, fn [first, second], {list1, list2} ->
    {[first | list1], [second | list2]}
  end)
  |> then(fn {list1, list2} ->
    {Enum.sort(list1), Enum.sort(list2)}
  end)
  |> then(fn {list1, list2} ->
    Enum.zip(list1, list2)
    |> Enum.map(fn {x, y} -> abs(x - y) end)
  end)
  |> Enum.sum()

part_two =
  File.read!("input.in")
  |> String.split("\n", trim: true)
  |> Enum.map(fn line ->
    line
    |> String.split()
    |> Enum.map(&String.to_integer/1)
  end)
  |> Enum.reduce({[], []}, fn [first, second], {list1, list2} ->
    {[first | list1], [second | list2]}
  end)
  |> then(fn {list1, list2} ->
    Enum.map(list1, fn line ->
      line * Enum.count(list2, fn x -> x === line end)
    end)
    |> Enum.sum()
  end)

IO.inspect(part_one)
IO.inspect(part_two)
