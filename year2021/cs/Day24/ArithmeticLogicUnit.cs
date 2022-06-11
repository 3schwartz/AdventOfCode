using LanguageExt.Common;

namespace Day24
{
    internal class ArithmeticLogicUnit
    {
        private IDictionary<string,int> processingUnits = new Dictionary<string, int>
            {
                {"w",0 },
                {"x",0 },
                {"y",0 },
                {"z",0 },

            };

        internal void Restore()
        {
            foreach (var item in processingUnits)
            {
                processingUnits[item.Key] = 0;
            }
        }

        internal (long Highest, IReadOnlyDictionary<string, int> Run) FindHighestMonad(string[] instructions)
        {
            for (long i = 99999999999999; i >= 11111111111111; i--)
            {
                var monad = i.ToString();
                if (monad.Contains('0')) continue;
                
                var run = RunMonad(instructions, monad);
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

        internal Result<IDictionary<string, int>> RunMonad(
            string[] instructions,
            string digits)
        {
            var idx = 0;
            foreach (var instruction in instructions)
            {
                var splitted = instruction.Split(' ');
                if(splitted[0] == "inp")
                {
                    processingUnits[splitted[1]] = digits[idx] - '0';
                    idx++;
                    continue;
                }

                var b = GetUnitInstruction(splitted[2]);
                var a = splitted[1];

                switch (splitted[0])
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
                        return new Result<IDictionary<string, int>>(new InvalidOperationException($"Unknown instruction {splitted[0]}"));
                }
            }

            return new Result<IDictionary<string, int>>(processingUnits);
        }

        private int GetUnitInstruction(string unit)
        {
            if(processingUnits.TryGetValue(unit, out var instruction))
            {
                return instruction;
            }

            return int.Parse(unit);
        }
    }
}
