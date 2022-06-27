using LanguageExt.Common;

namespace Day24
{
    internal class ArithmeticLogicUnit
    {
        private readonly IDictionary<string,int> processingUnits = new Dictionary<string, int>
            {
                {"w",0 },
                {"x",0 },
                {"y",0 },
                {"z",0 },

            };

        private void Restore()
        {
            foreach (var item in processingUnits)
            {
                processingUnits[item.Key] = 0;
            }
        }

        internal (long Highest, IReadOnlyDictionary<string, int> Run) FindHighestMonad(string[] instructions)
        {
            var listed = InitInstructions(instructions);

            for (long i = 99999999999999; i >= 11111111111111; i--)
            {
                if (i % 10_000_000 == 0) Console.WriteLine(i);

                var monad = i.ToString();
                if (monad.Contains('0')) continue;
                
                var run = RunMonad(listed, monad);
                var valid = run.Match(run =>
                {
                    if (run["z"] == 0)
                    {
                        return true;
                    }
                    return false;
                }, exception =>
                {
                    return false;
                });

                if (valid)
                {
                    return (i, (IReadOnlyDictionary<string, int>)processingUnits);
                }

                Restore();
            }

            throw new InvalidOperationException("Given instructions didn't gave a solution");
        }

        private (bool, string, int) SetLookup(string unit)
        {
            if (processingUnits.Keys.Contains(unit))
            {
                return (true, unit, 0);
            }

            return (false, unit, int.Parse(unit));
        }

        internal (IList<(string, string)> instructions, IDictionary<int, (bool, string, int)> lookup) InitInstructions(string[] instructions)
        {
            var lookup = new Dictionary<int, (bool, string, int)>();
            var listed = new List<(string,string)>(instructions.Length);

            for (int i = 0; i < instructions.Length; i++)
            {
                var splitted = instructions[i].Split(' ');
                
                listed.Add((splitted[0], splitted[1]));

                if (splitted[0] == "inp")
                {
                    continue;
                }

                lookup.Add(i, SetLookup(splitted[2]));
            }

            return (listed, lookup);
        }

        internal Result<IDictionary<string, int>> RunMonad(
            (IList<(string Instruct, string LookUp)> instructions, IDictionary<int, (bool, string, int)> lookup) instructions,
            string digits)
        {
            var idx = 0;
            for (int i = 0; i < instructions.instructions.Count; i++)
            {
                var (Instruct, LookUp) = instructions.instructions[i];
                
                if(Instruct == "inp")
                {
                    processingUnits[LookUp] = digits[idx] - '0';
                    idx++;
                    continue;
                }

                var b = GetUnitInstruction(instructions.lookup[i]);
                var a = LookUp;

                switch (Instruct)
                {

                    case "add":
                        processingUnits[a] += b;
                        continue;
                    case "mul":
                        processingUnits[a] *=  b;
                        continue;
                    case "div":
                        if (b == 0) return new Result<IDictionary<string, int>>(new InvalidOperationException("Can't divide by zero"));
                        processingUnits[a] = processingUnits[a] / b;
                        continue;
                    case "mod":
                        if (b <= 0) return new Result<IDictionary<string, int>>(new InvalidOperationException($"Can't do module with b: {b}"));
                        if (processingUnits[a] <= 0) return new Result<IDictionary<string, int>>(new InvalidOperationException($"Can't do module with a: {processingUnits[a]}"));
                        processingUnits[a] = processingUnits[a] % b;
                        continue;
                    case "eql":
                        processingUnits[a] = processingUnits[a] == b ? 1 : 0;
                        continue;
                    default:
                        return new Result<IDictionary<string, int>>(new InvalidOperationException($"Unknown instruction {Instruct}"));
                }
            }

            return new Result<IDictionary<string, int>>(processingUnits);
        }

        private int GetUnitInstruction((bool, string, int) p)
        {
            if (p.Item1)
            {
                return processingUnits[p.Item2];
            }
            return p.Item3;
        }
    }
}
