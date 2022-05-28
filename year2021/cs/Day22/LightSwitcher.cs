namespace Day22
{
    internal class LightSwitcher
    {
        private readonly string[] lines;

        public LightSwitcher(string[] lines)
        {
            this.lines = lines;
        }

        internal int GetOnLights(bool useLimit)
        {
            var lights = new HashSet<(int, int, int)>();
            foreach (var step in lines)
            {
                var instruction = Create(step);

                if (useLimit && !IsInstructionValid(instruction)) continue;

                for (var x = instruction.XFrom; x <= instruction.XTo; x++)
                {
                    for (var y = instruction.YFrom; y <= instruction.YTo; y++)
                    {
                        for (var z = instruction.ZFrom; z <= instruction.ZTo; z++)
                        {
                            switch (instruction.Turn)
                            {
                                case Turn.On:
                                    lights.Add((x, y, z));
                                    break;
                                case Turn.Off:
                                    lights.Remove((x, y, z));
                                    break;
                            }

                        }
                    }
                }
            }

            return lights.Count;
        }

        private static bool IsInstructionValid(Instruction instruction)
        {
            return instruction.XFrom >= -50 &&
                   instruction.YFrom >= -50 &&
                   instruction.ZFrom >= -50 &&
                   instruction.XTo <= 50 &&
                   instruction.YTo <= 50 &&
                   instruction.ZTo <= 50;
        }

        internal static Instruction Create(string line)
        {
            var initial = line.Split(" ");
            Enum.TryParse(initial[0], true, out Turn turn);

            var coordinates = initial[1].Split(",");
            var x = GetFromTo(coordinates[0]);
            var y = GetFromTo(coordinates[1]);
            var z = GetFromTo(coordinates[2]);
            return new Instruction(
                turn,
                x.Item1, x.Item2,
                y.Item1, y.Item2,
                z.Item1, z.Item2);
        }

        private static (int, int) GetFromTo(string coordinate)
        {
            var initial = coordinate.Split("=");
            var coordinates = initial[1].Split("..");
            return (int.Parse(coordinates[0]), int.Parse(coordinates[1]));
        }

        internal record struct Instruction(Turn Turn, int XFrom, int XTo, int YFrom, int YTo, int ZFrom, int ZTo);
    }
}
