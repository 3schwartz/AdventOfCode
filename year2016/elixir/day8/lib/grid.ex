defmodule Grid do
  def get_row(map, row) do
    map
    |> Enum.filter(fn {{m, _}, _} -> m == row end)
    |> Enum.sort(fn {{_, n1}, _}, {{_, n2}, _} -> n1 < n2 end)
  end

  def get_column(map, col) do
    map
    |> Enum.filter(fn {{_, n}, _} -> n == col end)
    |> Enum.sort(fn {{m1, _}, _}, {{m2, _}, _} -> m1 < m2 end)
  end

  def update(list, map) do
    Enum.reduce(list, map, fn {c, e}, m -> Map.put(m, c, e) end)
  end

  def rotate(list, 0), do: list

  def rotate([first | rest], count) do
    rotate(rest ++ [first], count - 1)
  end

  def rotate_row(list, count) do
    rotate(list, length(list) - count)
    |> Enum.with_index()
    |> Enum.map(fn {{{m, _}, e}, index} ->
      {{m, index}, e}
    end)
  end

  def rotate_column(list, count) do
    rotate(list, length(list) - count)
    |> Enum.with_index()
    |> Enum.map(fn {{{_, n}, e}, index} ->
      {{index, n}, e}
    end)
  end

  def turn_on_lights(map, column, row) do
    Enum.reduce(0..(row - 1), map, fn m, acc_map ->
      Enum.reduce(0..(column - 1), acc_map, fn n, acc_map ->
        Map.put(acc_map, {m, n}, "#")
      end)
    end)
  end

  def parse_numbers(rest, seperator) do
    [first, last] = String.split(rest, seperator)
    {String.to_integer(first), String.to_integer(last)}
  end

  def create_empty(row, column) do
    Enum.reduce(0..(row - 1), %{}, fn m, acc_map ->
      Enum.reduce(0..(column - 1), acc_map, fn n, acc_map ->
        Map.put(acc_map, {m, n}, ".")
      end)
    end)
  end

  def parse_command(map, "rotate column x=" <> rest) do
    {col, count} = parse_numbers(rest, " by ")

    get_column(map, col)
    |> rotate_column(count)
    |> update(map)
  end

  def parse_command(map, "rotate row y=" <> rest) do
    {row, count} = parse_numbers(rest, " by ")

    get_row(map, row)
    |> rotate_row(count)
    |> update(map)
  end

  def parse_command(map, "rect " <> rest) do
    {column, row} = parse_numbers(rest, "x")
    turn_on_lights(map, column, row)
  end

  def print(map, row, col) do
    0..(row - 1)
    |> Enum.each(fn r ->
      0..(col - 1)
      |> Enum.each(fn c -> IO.write("#{Map.get(map, {r, c})}") end)

      IO.puts("\n")
    end)

    IO.puts("\n")
    IO.puts("\n")
    map
  end
end
