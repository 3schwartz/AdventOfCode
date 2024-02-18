"Hello " <> to_string(Day1.hello()) <> "!"
|> IO.puts

lines =
  File.stream!("../../data/day1_data.txt")
  |> Stream.map(&String.split(&1, ", "))
  |> Enum.to_list

Enum.each(lines, &IO.puts/1)
