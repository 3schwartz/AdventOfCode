lines =
  File.stream!("../../data/day1_data.txt")
  |> Stream.map(&String.split(&1, ", "))
  |> Enum.flat_map(&(&1))


{{x, y}, _} = Enum.reduce(lines, {{0, 0}, {0, 1}}, fn move, state -> Day1.apply_action(move, state) end)

IO.puts("Part 1: #{Day1.manhattan_distance({x, y})}")

{{x, y}, _, _} = Enum.reduce_while(lines, {{0, 0}, {0, 1}, MapSet.new()}, fn move, state -> Day1.apply_action_with_set(move, state) end)

IO.puts("Part 2: #{Day1.manhattan_distance({x, y})}")
