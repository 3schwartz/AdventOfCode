namespace Day22
{
    internal class LightSwitcher
    {
        private readonly bool useLimit;
        public LightSwitcher()
        {
            useLimit = true;
        }

        public LightSwitcher(bool useLimit)
        {
            this.useLimit = useLimit;
        }
        private bool IsInstructionValid(Instruction instruction)
        {
            return instruction.XFrom >= -50 &&
                   instruction.YFrom >= -50 &&
                   instruction.ZFrom >= -50 &&
                   instruction.XTo <= 50 &&
                   instruction.YTo <= 50 &&
                   instruction.ZTo <= 50;
        }

        internal int GetOnLights(string[] steps)
        {
            var lights = new HashSet<(int, int, int)>();
            foreach (var step in steps)
            {
                var instruction = InstructionCreator.Create(step);

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
    }
}
