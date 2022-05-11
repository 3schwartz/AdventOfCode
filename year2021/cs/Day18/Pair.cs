using System.Collections.Immutable;
using System.Text;

namespace Day18
{
    internal class Pair
    {
        const char LeftSquareBracket = '[';
        const char RightSquareBracket = ']';
        const char Comma = ',';
        private ImmutableList<char> splitters => ImmutableList.Create<char>(
            LeftSquareBracket, RightSquareBracket, Comma);

        public Pair? LeftPair { get; private set; }
        public Pair? RightPair { get; private set; }
        public int? LeftNumber { get; private set; }
        public int? RightNumber { get; private set; }
        private Pair? AbovePair { get; set; }
        private int Debt { get; set; }
        private Position PairPosition { get; set; }

        internal Pair(string input)
        {
            var span = input.AsSpan();
            Debt = 0;
            PairPosition = Position.Left;
            FindNext(span);
        }

        internal Pair(int debt, Position type, Pair above)
        {
            Debt = debt;
            PairPosition = type;
            AbovePair = above;
        }

        internal Pair(int debt, Position type, Pair above, int rightNumber, int leftNumber)
        {
            Debt = debt;
            PairPosition = type;
            AbovePair = above;
            RightNumber = rightNumber;
            LeftNumber = leftNumber;
        }

        public Pair(Pair leftPair, Pair rightPair)
        {
            LeftPair = leftPair;
            RightPair = rightPair;
            LeftPair.AbovePair = this;
            RightPair.AbovePair = this;
            Debt = 0;
        }

        public override string ToString()
        {
            var builder = new StringBuilder();
            builder.Append("[");
            if (LeftNumber != null)
            {
                builder.Append(LeftNumber);
            }
            if (LeftPair != null)
            {
                builder.Append(LeftPair.ToString());
            }

            builder.Append(",");
            if (RightNumber != null)
            {
                builder.Append(RightNumber);
            }
            if (RightPair != null)
            {
                builder.Append(RightPair.ToString());
            }
            builder.Append("]");
            return builder.ToString();
        }

        internal static int CalculateHomework(IList<Pair> pairs)
        {
            var number = pairs[0];
            for (var i = 1; i < pairs.Count; i++)
            {
                number = number.Add(pairs[i]);
                number.Reduce();
            }

            return number.CalculateMagnitude();
        }

        internal Pair Add(Pair pair)
        {
            PairPosition = Position.Left;
            pair.PairPosition = Position.Right;
            IncreaseDebt();
            pair.IncreaseDebt();
            return new Pair(this, pair);
        }

        internal void Reduce()
        {
            //bool reduced;
            //do
            //{
            //    reduced = ReduceCurrent();
            //} while (reduced);
            do
            {
                if (ReduceCurrentUpdate())
                {
                    continue;
                }

                break;
            } while (true);
        }

        internal bool ReduceCurrentUpdate()
        {
            if (ExplodeCurrent())
            {
                return true;
            }

            if (SplitCurrent())
            {
                return true;
            }

            return false;
        }

        internal int CalculateMagnitude()
        {
            var leftNumber = (int)(LeftPair?.CalculateMagnitude() ?? LeftNumber);
            var rightNumber = (int)(RightPair?.CalculateMagnitude() ?? RightNumber);

            return 3 * leftNumber + 2 * rightNumber;
        }

        private bool ExplodeCurrent()
        {
            if (Debt > 3 && LeftNumber != null && RightNumber != null)
            {
                Explode();
                return true;
            }
            if (LeftPair != null && LeftPair.ExplodeCurrent())
            {
                return true;
            }
            if (RightPair != null && RightPair.ExplodeCurrent())
            {
                return true;
            }
            return false;
        }

        private bool SplitCurrent()
        {
            if (LeftNumber > 9)
            {
                Split(Position.Left, LeftNumber);
                return true;
            }
            if (LeftPair != null && LeftPair.SplitCurrent())
            {
                return true;
            }
            if (RightNumber > 9)
            {
                Split(Position.Right, RightNumber);
                return true;
            }
            if (RightPair != null && RightPair.SplitCurrent())
            {
                return true;
            }
            return false;
        }

        [Obsolete]
        internal bool ReduceCurrent()
        {
            if (Debt > 3  && LeftNumber != null && RightNumber != null)
            {
                Explode();
                return true;
            }
            if(LeftNumber > 9)
            {
                Split(Position.Left, LeftNumber);
                return true;
            }
            if (RightNumber > 9)
            {
                Split(Position.Right, RightNumber);
                return true;
            }
            if (LeftPair != null && LeftPair.ReduceCurrent())
            {
                return true;
            }
            if (RightPair != null && RightPair.ReduceCurrent())
            {
                return true;
            }
            return false;
        }

        private void IncreaseDebt()
        {
            Debt += 1;
            LeftPair?.IncreaseDebt();
            RightPair?.IncreaseDebt();
        }

        private void Split(Position position, int? number)
        {
            var leftNumber = (int)Math.Floor((double)number / 2);
            var rightNumber = (int)Math.Ceiling((double)number / 2);

            var newPair = new Pair(Debt + 1, position, this, rightNumber, leftNumber);

            switch (position)
            {
                case Position.Left:
                    LeftNumber = null;
                    LeftPair = newPair;
                    break;
                case Position.Right:
                    RightNumber = null;
                    RightPair = newPair;
                    break;
            }
        }

        private void Explode()
        {
            AbovePair?.AddAboveLeft(PairPosition, LeftNumber);
            AbovePair?.AddAboveRight(PairPosition, RightNumber);

            switch (PairPosition)
            {
                case Position.Left:
                    if (AbovePair?.LeftNumber == null)
                    {
                        AbovePair.LeftNumber = 0;
                    }
                    AbovePair.LeftPair = null;
                    break;
                case Position.Right:
                    if (AbovePair?.RightNumber == null)
                    {
                        AbovePair.RightNumber = 0;
                    }
                    AbovePair.RightPair = null;
                    break;
            }
        }

        private void AddAboveRight(Position position, int? rightNumber)
        {
            if (RightNumber != null)
            {
                RightNumber += rightNumber;
                return;
            }
            if (position == Position.Left && RightPair != null)
            {
                RightPair.AddBelowLeft(rightNumber);
                return;
            }

            AbovePair?.AddAboveRight(PairPosition, rightNumber);
        }

        private void AddAboveLeft(Position position, int? leftNumber)
        {
            if (LeftNumber != null)
            {
                LeftNumber += leftNumber;
                return;
            }

            if(position == Position.Right && LeftPair != null)
            {
                LeftPair.AddBelowRight(leftNumber);
                return;
            }

            AbovePair?.AddAboveLeft(PairPosition, leftNumber);
        }

        private void AddBelowLeft(int? rightNumber)
        {
            if(LeftNumber != null)
            {
                LeftNumber += rightNumber;
                return;
            }
            LeftPair?.AddBelowLeft(rightNumber);
        }

        private void AddBelowRight(int? leftNumber)
        {
            if (RightNumber != null)
            {
                RightNumber += leftNumber;
                return;
            }
            RightPair?.AddBelowRight(leftNumber);
        }

        private void FindNext(ReadOnlySpan<char> span)
        {
            switch (span[1])
            {
                case LeftSquareBracket:
                    if (LeftPair == null && LeftNumber == null)
                    {
                        LeftPair = new Pair(Debt + 1, Position.Left, this);
                        LeftPair.FindNext(span[1..]);
                        break;
                    }
                    RightPair = new Pair(Debt + 1, Position.Right, this);
                    RightPair.FindNext(span[1..]);
                    break;
                case RightSquareBracket:
                    if (Debt > 0)
                    {
                        AbovePair?.FindNext(span[1..]);
                    }
                    //AbovePair?.FindNext(span[1..]);
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

        public override bool Equals(object? obj)
        {
            if (typeof(object).Equals(typeof(Pair))) return false;
            var number = (Pair)obj;

            bool equals = LeftNumber == number?.LeftNumber && this?.RightNumber == number?.RightNumber;

            equals &= (AbovePair == null) == (number?.AbovePair == null);

            equals &= PairPosition == number?.PairPosition;

            if (LeftPair != null)
            {
                equals &= LeftPair.Equals(number?.LeftPair);
            }

            if (RightPair != null)
            {
                equals &= RightPair.Equals(number?.RightPair);
            }

            return equals;
        }

        public override int GetHashCode()
        {
            return base.GetHashCode();
        }
    }
}
