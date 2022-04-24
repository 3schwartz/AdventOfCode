namespace Day14
{
    internal class PolymerInserter
    {    
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

        internal Span<char> ApplyFoundRules(ReadOnlySpan<char> template, IList<RulesToApply> rulesToApply)
        {
            Span<char> result = new char[rulesToApply.Count + template.Length];
            var inserted = 0;
            var nextIndex = 0;
            var lastKey = 0;
            foreach (var pair in rulesToApply)
            {
                var key = pair.Index + inserted;
                template[lastKey..pair.Index].CopyTo(result[nextIndex..key]);
                result[key] = pair.Insert;

                lastKey = pair.Index;
                nextIndex = key + 1;
                inserted++;
            }

            template[lastKey..].CopyTo(result[nextIndex..]);

            return result;
        }

        internal IList<RulesToApply> FindRulesToApply(ReadOnlySpan<char> template, IList<Rule> rules)
        {
            var rulesToApply = new List<RulesToApply>();
            for (int i = 0; i < template.Length - 1; i++)
            {
                foreach (var rule in rules)
                {
                    if (template[i] == rule.First && template[i + 1] == rule.Second)
                    {
                        rulesToApply.Add(new RulesToApply(i + 1, rule.Insert));
                    }
                }
            }
            rulesToApply.Sort();

            return rulesToApply;
        }

        internal struct RulesToApply : IEquatable<RulesToApply>, IComparable<RulesToApply>
        {
            public int Index { get; private set; }
            public char Insert { get; private set; }

            public RulesToApply(int index, char insert)
            {
                Index = index;
                Insert = insert;
            }

            public int CompareTo(RulesToApply other)
            {
                return other.Index > Index ? -1 :
                    other.Index == Index ? 0 : 1;
            }

            public bool Equals(RulesToApply other)
            {
                return other.Insert == Insert && other.Index == Index;
            }
        }
    }
}
