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

  def update(map, list) do
    Enum.reduce(list, map, fn {c, e}, m -> Map.put(m, c, e) end)
  end

  def rotate(list, 0), do: list
  def rotate([first | rest], count) do
    rotate(rest ++ [first], count - 1)
  end

  def rotate_row(list, count) do
    rotate(list, length(list) - count)
      |> Enum.with_index()
      |> Enum.map(fn {{{_, n}, e}, index} ->
        {{index, n}, e}
      end)
  end

  def rotate_column(list, count) do
    rotate(list, length(list) - count)
      |> Enum.with_index()
      |> Enum.map(fn {{{m, _}, e}, index} ->
        {{m, index}, e}
      end)
  end

  def turn_on_lights(map, row, column) do
    Enum.reduce(0..row, map, fn m, acc_map ->
      Enum.reduce(0..column, acc_map, fn n, acc_map ->
        Map.put(acc_map, {m, n}, "#")
      end)
    end)
  end
end
