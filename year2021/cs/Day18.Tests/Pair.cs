using System;
using System.Collections.Immutable;

namespace Day18.Tests
{
    internal class Pair
    {
        const char LeftSquareBracket = '[';
        const char RightSquareBracket = ']';
        const char Comma = ',';
        private ImmutableList<char> splitters => ImmutableList.Create<char>(
            LeftSquareBracket, RightSquareBracket, Comma);

        public Pair? LeftPair { get; set; }
        public Pair? RightPair { get; set; }
        public int? LeftNumber { get; set; }
        public int? RightNumber { get; set; }
        public Pair? AbovePair { get; set; }
        public int Debt { get; set; }
        public Position PairPosition { get; }

        internal Pair(string input)
        {
            var span = input.AsSpan();
            Debt = 0;
            PairPosition = Position.Left;
            FindNext(span);
        }

        internal Pair(int debt, Position type, Pair above, ReadOnlySpan<char> span)
        {
            Debt = debt;
            PairPosition = type;
            AbovePair = above;
            FindNext(span);
        }

        private void FindNext(ReadOnlySpan<char> span)
        {
            switch (span[1])
            {
                case LeftSquareBracket:
                    if (LeftPair == null && LeftNumber == null)
                    {
                        LeftPair = new Pair(Debt + 1, Position.Left, this, span[1..]);
                        break;
                    }
                    RightPair = new Pair(Debt + 1, Position.Right, this, span[1..]);
                    break;
                case RightSquareBracket:
                    if(Debt > 0)
                    {
                        AbovePair.FindNext(span[1..]);
                    }
                    break;
                case Comma:
                    FindNext(span[1..]);
                    break;
                default:
                    int size = splitters.Contains(span[2]) ? 1 : 2;
                    var number = int.Parse(span.Slice(1, size));
                    if (span[0] == Comma)
                    {
                        RightNumber = number;
                    }
                    else
                    {
                        LeftNumber = number;
                    }
                    FindNext(span[size..]);
                    break;
            }
        }

        internal enum Position
        {
            Left,
            Right
        }
    }
}
