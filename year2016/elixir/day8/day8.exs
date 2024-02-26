lines =
  File.stream!("../../data/day8_data.txt")
  |> Stream.map(&String.trim_trailing/1)
  |> Enum.to_list()

result = Enum.reduce(lines, Grid.create_empty(6, 50), fn line, map ->
  IO.puts(line)
  Grid.parse_command(map, line)
    |> Grid.print(6, 50)
end)
  |> Enum.reduce(0, fn {_, v}, sum->
    if v == "#" do
      sum + 1
    else
      sum
    end
  end)

IO.puts("Part 1 #{result}")
