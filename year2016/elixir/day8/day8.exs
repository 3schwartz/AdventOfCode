lines =
  File.stream!("../../data/day8_data.txt")
  |> Stream.map(&String.trim_trailing/1)
  |> Enum.to_list()


grid = Enum.reduce(lines, Grid.create_empty(6, 50), fn line, map ->
    Grid.parse_command(map, line)
  end)

result = grid
 |> Enum.reduce(0, fn {_, v}, sum->
    if v == "#" do
      sum + 1
    else
      sum
    end
  end)

IO.puts("Part 1 #{result}")

grid
  |> Grid.print(6, 50)
