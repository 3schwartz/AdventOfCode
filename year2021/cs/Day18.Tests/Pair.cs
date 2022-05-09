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
            Debt = 0;
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
            bool reduced;
            do
            {
                reduced = ReduceCurrent();
            } while (reduced);
        }

        internal int CalculateMagnitude()
        {
            var leftNumber = (int)(LeftPair?.CalculateMagnitude() ?? LeftNumber);
            var rightNumber = (int)(RightPair?.CalculateMagnitude() ?? RightNumber);

            return 3 * leftNumber + 2 * rightNumber;
        }

        private bool ReduceCurrent()
        {
            if (Debt > 3 )
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
            if(AbovePair?.LeftNumber == null)
            {
                AbovePair.LeftNumber = 0;
            }
            AbovePair.LeftPair = null;

            AbovePair?.AddAboveRight(PairPosition, RightNumber);
            if (AbovePair?.RightNumber == null)
            {
                AbovePair.RightNumber = 0;
            }
            AbovePair.RightPair = null;
        }

        private void AddAboveRight(Position position, int? rightNumber)
        {
            //switch (position)
            //{
            //    case Position.Left:
            //        if (LeftPair != null)
            //        {
            //            LeftPair.AddAboveRight(PairPosition, rightNumber);
            //            break;
            //        }
            //        if (LeftNumber != null)
            //        {
            //            LeftNumber += rightNumber;
            //            break;
            //        }
            //        RightNumber += rightNumber;
            //        break;
            //    case Position.Right:
            //        if (RightNumber != null)
            //        {
            //            RightNumber += rightNumber;
            //            break;
            //        }
            //        if (PairPosition == Position.Left)
            //        {
            //            RightPair.AddAboveRight(position, rightNumber);
            //            break;
            //        }
            //        AbovePair.AddAboveRight(position, rightNumber);
            //        break;
            //}

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
            }

            AbovePair?.AddAboveLeft(PairPosition, leftNumber);
        }

        private void AddBelowLeft(int? rightNumber)
        {
            if(LeftNumber != null)
            {
                LeftNumber += rightNumber;
            }
            LeftPair?.AddBelowLeft(rightNumber);
        }

        private void AddBelowRight(int? leftNumber)
        {
            if (RightNumber != null)
            {
                RightNumber += leftNumber;
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
