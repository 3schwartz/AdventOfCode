namespace Day14
{
    public class PolymerInserter
    {

        internal string[] GetData(string path)
        {
            return File.ReadAllLines(path);
        }

        internal IList<Rule> CreateRules(string[] lineRules)
        {
            var rules = new List<Rule>(lineRules.Length);

            foreach (var rule in lineRules)
            {
                var span = rule.AsSpan();
                rules.Add(new Rule(span[0], span[1], span[^1]));
            }

            return rules;
        }

        internal ReadOnlySpan<char> DoInsertion(string[] data, int insertionCount)
        {
            var polymerTemplate = data[0];

            var rules = CreateRules(data[2..]);

            var iterations = 0;
            var template = polymerTemplate.AsSpan();
            while (iterations < insertionCount)
            {
                template = DoCycle(template, rules);
                iterations++;
            }

            return template;
        }

        internal record struct Rule(char First, char Second, char Insert);

        internal Span<char> DoCycle(ReadOnlySpan<char> template, IList<Rule> rules)
        {
            var rulesToApply = FindRulesToApply(template, rules);

            return ApplyFoundRules(template, rulesToApply);
        }

        internal int GetMostCommonMinusLeastCommon(ReadOnlySpan<char> polymerTemplate)
        {
            var counts = new Dictionary<char, int>();

            foreach(var c in polymerTemplate)
            {
                counts[c] = counts.TryGetValue(c, out var count) ? count + 1 : 1;
            }

            return counts.Values.Max() - counts.Values.Min();
        }

        internal Span<char> ApplyFoundRules(ReadOnlySpan<char> template, SortedDictionary<int, char> rulesToApply)
        {
            Span<char> result = new char[rulesToApply.Count + template.Length];
            var inserted = 0;
            var nextIndex = 0;
            var lastKey = 0;
            foreach (var pair in rulesToApply)
            {
                var key = pair.Key + inserted;
                template[lastKey..pair.Key].CopyTo(result[nextIndex..key]);
                result[key] = pair.Value;

                lastKey = pair.Key;
                nextIndex = key + 1;
                inserted++;
            }

            template[lastKey..].CopyTo(result[nextIndex..]);

            return result;
        }

        internal SortedDictionary<int, char> FindRulesToApply(ReadOnlySpan<char> template, IList<Rule> rules)
        {
            var rulesToApply = new SortedDictionary<int, char>();
            for (int i = 0; i < template.Length - 1; i++)
            {
                foreach (var rule in rules)
                {
                    if (template[i] == rule.First && template[i + 1] == rule.Second)
                    {
                        rulesToApply[i + 1] = rule.Insert;
                    }
                }
            }

            return rulesToApply;
        }

        internal struct Apply : IEquatable<Apply>, IComparable<Apply>
        {
            public int Index { get; private set; }
            public char Insert { get; private set; }

            public Apply(int index, char insert)
            {
                Index = index;
                Insert = insert;
            }

            public int CompareTo(Apply other)
            {
                return other.Index > Index ? 1 :
                    other.Index == Index ? 0 : -1;
            }

            public bool Equals(Apply other)
            {
                return other.Insert == Insert && other.Index == Index;
            }
        }
    }
}
