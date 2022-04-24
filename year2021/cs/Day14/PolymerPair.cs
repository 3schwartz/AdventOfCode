namespace Day14
{
    internal class PolymerPair
    {
        internal IList<DictRule> CreateDictRules(string[] lineRules)
        {
            var rules = new List<DictRule>(lineRules.Length);

            foreach (var rule in lineRules)
            {
                var span = rule.AsSpan();
                rules.Add(new DictRule(
                    span[..2].ToString(),
                    FirstInsert: new string(new char[2] { span[0], span[^1] }),
                    SecondInsert: new string(new char[2] { span[^1], span[1] }),
                     span[^1])
                    );
            }

            return rules;
        }

        internal IDictionary<char, long> CreateCharCounts(string template, IList<DictRule> rules)
        {
            var charCounts = new Dictionary<char, long>();
            foreach (var c in template)
            {
                if (charCounts.ContainsKey(c))
                {
                    charCounts[c] += 1;
                }
                else
                {
                    charCounts[c] = 1;
                }
            }
            foreach (var rule in rules)
            {
                if (!charCounts.ContainsKey(rule.Inject))
                {
                    charCounts[rule.Inject] = 0;
                }
            }
            return charCounts;
        }

        internal IDictionary<string, long> CreatePairs(string template, IList<DictRule> rules)
        {
            var pairs = new Dictionary<string, long>(rules.Count + template.Length - 1);
            foreach (var rule in rules)
            {
                pairs[rule.Rule] = 0;
                pairs[rule.FirstInsert] = 0;
                pairs[rule.SecondInsert] = 0;
            }

            var span = template.AsSpan();
            for (int i = 0; i < span.Length - 1; i++)
            {
                var pair = span.Slice(i, 2).ToString();
                if (pairs.TryGetValue(pair, out _))
                {
                    pairs[pair] += 1;
                }
                else
                {
                    pairs[pair] = 1;
                }
            }

            return pairs;
        }

        internal void UpdatePairsFromRules(IDictionary<string, long> pairs,
            List<PairUpdate> rules, IDictionary<char, long> charCounts)
        {
            foreach (var rule in rules)
            {
                rule.PairCount = pairs[rule.Rule.Rule];
            }

            foreach (var rule in rules)
            {
                pairs[rule.Rule.Rule] -= rule.PairCount;
                pairs[rule.Rule.FirstInsert] += rule.PairCount;
                pairs[rule.Rule.SecondInsert] += rule.PairCount;
                charCounts[rule.Rule.Inject] += rule.PairCount;
            }
        }

        internal IDictionary<char, long> UpdatePairs(string[] data, int insertions)
        {
            var rules = CreateDictRules(data[2..]);
            var pairs = CreatePairs(data[0], rules);
            var charCounts = CreateCharCounts(data[0], rules);

            var updates = new List<PairUpdate>(rules.Count);
            foreach (var rule in rules)
            {
                updates.Add(new PairUpdate(rule, pairs[rule.Rule]));
            }

            var count = 0;
            while (count < insertions)
            {
                UpdatePairsFromRules(pairs, updates, charCounts);
                count++;
            }

            return charCounts;
        }

        internal long MostMinusLeastFromPairs(IDictionary<char, long> charCounts)
        {
            return charCounts.Values.Max() - charCounts.Values.Min();
        }

        internal record struct DictRule(string Rule, string FirstInsert, string SecondInsert, char Inject);

        internal class PairUpdate
        {
            internal PairUpdate(DictRule Rule, long PairCount)
            {
                this.Rule = Rule;
                this.PairCount = PairCount;
            }

            public DictRule Rule { get; }
            public long PairCount { get; set; }
        }

    }
}
