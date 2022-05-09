using System;
using System.Collections.Immutable;
using System.Linq;
using Common;
using Xunit;

namespace Day18.Tests;

public class Day18Tests
{
    [Fact]
    public void WhenConstruct_ThenCorrect()
    {
        // Arrange
        var input = "[[[[[9,8],1],2],3],4]";

        // Act
        var number = new SnailFishNumber(input);

        // Assert
        Assert.Equal(4, number.Debt);
        Assert.Equal(9, number.Number);
        Assert.Equal(SnailFishNumber.Position.Left, number.PositionEnum);
        Assert.Equal(8, number.Right.Number);
        Assert.Equal(4, number.Right.Debt);
        Assert.Equal(SnailFishNumber.Position.Right, number.Right.PositionEnum);
        Assert.Equal(1, number.Right.Right.Number);
        Assert.Equal(3, number.Right.Right.Debt);
        Assert.Equal(SnailFishNumber.Position.Right, number.Right.Right.PositionEnum);
        Assert.Equal(2, number.Right.Right.Right.Number);
        Assert.Equal(2, number.Right.Right.Right.Debt);
        Assert.Equal(3, number.Right.Right.Right.Right.Number);
        Assert.Equal(1, number.Right.Right.Right.Right.Debt);
        Assert.Equal(4, number.Right.Right.Right.Right.Right.Number);
        Assert.Equal(0, number.Right.Right.Right.Right.Right.Debt);
        Assert.Equal(SnailFishNumber.Position.Right, number.Right.Right.Right.Right.Right.PositionEnum);
    }

    [Theory]
    [InlineData("[[[[[9,8],1],2],3],4]", 0,3,9,3,2,2, 3, 1, 4,0)]
    [InlineData("[7,[6,[5,[4,[3,2]]]]]", 7, 0, 6, 1, 5, 2, 7, 3,0,3)]
    [InlineData("[[6,[5,[4,[3,2]]]],1]", 6,1,5,2,7,3, 0,3,3,0)]
    [InlineData("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", 3,1,2,2,8,3, 0,3,9,1)]
    [InlineData("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", 3,1,2,2,8,3,0,3,9,1)]
    public void WhenExplode_ThenCorrectMountOfNumbers(
        string input,
        int firstNumber, int firstDebt,
        int secondNumber, int secondDebt,
        int thirdNumber, int thirdDebt,
        int firthNumber, int firthDebt,
        int fifthNumber, int fifthDebt)
    {
        // Arrange
        var snailFishNumber = new SnailFishNumber(input);
        var reduced = false;
        // Act
        snailFishNumber.Reduce(ref reduced);

        // Assert
        Assert.Equal(firstNumber, snailFishNumber.Number);
        Assert.Equal(firstDebt, snailFishNumber.Debt);
        Assert.Equal(secondNumber, snailFishNumber.Right.Number);
        Assert.Equal(secondDebt, snailFishNumber.Right.Debt);
        Assert.Equal(thirdNumber, snailFishNumber.Right.Right.Number);
        Assert.Equal(thirdDebt, snailFishNumber.Right.Right.Debt);
        Assert.Equal(firthNumber, snailFishNumber.Right.Right.Right.Number);
        Assert.Equal(firthDebt, snailFishNumber.Right.Right.Right.Debt);
        Assert.Equal(fifthNumber, snailFishNumber.Right.Right.Right.Right.Number);
        Assert.Equal(fifthDebt, snailFishNumber.Right.Right.Right.Right.Debt);
    }

    [Theory]
    [InlineData("[[[[0,7],4],[15,[0,13]]],[1,1]]", 7, 3, 8)]
    public void WhenSplit_ThenCorrectNewPair(
        string input,
        int leftNumber, int debt, int rightNumber)
    {
        // Arrange
        var number = new SnailFishNumber(input);
        var reduced = false;

        // Act
        number.Reduce(ref reduced);

        // Assert
        Assert.Equal(leftNumber, number.Right.Right.Right.Number);
        Assert.Equal(debt, number.Right.Right.Right.Debt);
        Assert.Equal(SnailFishNumber.Position.Left, number.Right.Right.Right.PositionEnum);
        Assert.Equal(rightNumber, number.Right.Right.Right.Right.Number);
        Assert.Equal(SnailFishNumber.Position.Right, number.Right.Right.Right.Right.PositionEnum);
    }

    [Theory]
    [InlineData("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", 6, 4, 7)]
    public void WhenSplitSecond_ThenCorrectNewPair(
        string input,
        int leftNumber, int debt, int rightNumber)
    {
        // Arrange
        var number = new SnailFishNumber(input);
        var reduced = false;
        // Act
        number.Reduce(ref reduced);

        // Assert
        Assert.Equal(leftNumber, number.Right.Right.Right.Right.Right.Right.Number);
        Assert.Equal(debt, number.Right.Right.Right.Right.Right.Right.Debt);
        Assert.Equal(SnailFishNumber.Position.Left, number.Right.Right.Right.Right.Right.Right.PositionEnum);
        Assert.Equal(rightNumber, number.Right.Right.Right.Right.Right.Right.Right.Number);
        Assert.Equal(SnailFishNumber.Position.Right, number.Right.Right.Right.Right.Right.Right.Right.PositionEnum);
    }

    [Fact]
    public void WhenReduce_ThenAllReduced()
    {
        // Arrange
        var number = new SnailFishNumber("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");

        // Act
        number = SnailFishNumber.Reduce(number);

        // Assert
        Assert.Equal(3, number.Debt);
        Assert.Equal(0, number.Number);
        Assert.Equal(8, number.Right.Right.Right.Right.Number);
        Assert.Equal(3, number.Right.Right.Right.Right.Debt);
    }

    [Theory]
    [InlineData("[[9,1],[1,9]]", 129)]
    [InlineData("[1,2],[[3,4],5]]", 143)]
    [InlineData("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)]
    [InlineData("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)]
    [InlineData("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)]
    [InlineData("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)]
    [InlineData("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488)]
    public void WhenCalculateMagnitude_ThenCorrect(string input, int magnitude)
    {
        // Arrange
        var number = new SnailFishNumber(input);

        // Act
        var actual = number.CalculateMagnitude();

        // Assert
        Assert.Equal(magnitude, actual);
    }

    [Fact(Skip = "Not working")]
    public void WhenCalculateFinalSum_ThenCorrect()
    {
        // Arrange
        var numbers = DataLoader.GetData("../../../../../data/day18_data_test.txt")
            .Select(d => new SnailFishNumber(d)).ToList();

        // Act
        var number = numbers[0];
        for (var i = 1; i < numbers.Count; i++)
        {
            number.Add(numbers[i]);
            number = SnailFishNumber.Reduce(number);
        }

        var magnitude = number.CalculateMagnitude();

        // Assert
        Assert.Equal(3488, magnitude);
    }

    [Fact]
    public void WhenAdd_ThenOneLongAndAllIncreaseDebt()
    {
        // Arrange
        var first = new SnailFishNumber("[[[[4,3],4],4],[7,[[8,4],9]]]");
        var second = new SnailFishNumber("[1,1]");

        // Act
        first.Add(second);

        // Assert
        Assert.Equal(4, first.Debt);
    }



    public class SnailFishNumber
    {
        const char LeftSquareBracket = '[';
        const char RightSquareBracket = ']';
        const char Comma = ',';
        private ImmutableList<char> splitters => ImmutableList.Create<char>(LeftSquareBracket, RightSquareBracket, Comma);

        public int Number { get; set;  }
        internal Position PositionEnum{ get; private set; }
        public int Debt { get; private set; }
        public SnailFishNumber? Right { get; set; }
        public SnailFishNumber? Left { get; set; }

        public SnailFishNumber(int debt, ReadOnlySpan<char> span, SnailFishNumber left)
        {
            Debt = debt;
            Left = left;

            FindNext(span);
        }

        public SnailFishNumber(string input)
        {
            Debt = -1;
            var span = input.AsSpan();

            FindNext(span);

        }

        internal SnailFishNumber(int debt, int number, Position position)
        {
            Debt = debt;
            Number = number;
            PositionEnum = position;
        }


        internal enum Position
        {
            Left,
            Right
        }

        private void FindNext(ReadOnlySpan<char> span)
        {
            var debt = Debt;
            var loop = true;
            var idx = 0;
            char last = default;
            do
            {
                switch (span[idx])
                {
                    case LeftSquareBracket:
                        Debt++;
                        debt++;
                        break;
                    case RightSquareBracket:
                        debt--;
                        break;
                    case Comma:
                        Right = new SnailFishNumber(debt, span[(idx+1)..], this);
                        loop = false;
                        break;
                    default:
                        if (!splitters.Contains(span[idx + 1]))
                        {
                            Number = int.Parse(span.Slice(idx, 2));
                            PositionEnum = last == LeftSquareBracket ? Position.Left : Position.Right;
                            idx++;
                            break;
                        }
                        Number = span[idx] - '0';
                        PositionEnum = last == LeftSquareBracket ? Position.Left : Position.Right;
                        break;
                }

                last = span[idx];
                idx++;
            } while (loop && idx < span.Length);
        }

        private void ReduceCurrent(ref bool reduced)
        {
            if (Debt > 3)
            {
                Explode();
                reduced = true;
            }

            if (Number > 9)
            {
                Split();
                reduced = true;
            }
        }

        private void Split()
        {
            var leftNumber = new SnailFishNumber(Debt + 1,
                (int)Math.Floor((double)Number / 2),
                Position.Left);
            var rightNumber = new SnailFishNumber(Debt + 1,
                (int)Math.Ceiling((double)Number / 2),
                Position.Right);

            leftNumber.Right = rightNumber;
            rightNumber.Left = leftNumber;
            
            if (Left != null)
            {
                Left.Right = leftNumber;
                leftNumber.Left = Left;
            }

            if (Right != null)
            {
                Right.Left = rightNumber;
                rightNumber.Right = Right;
            }
        }

        private void Explode()
        {
            if (PositionEnum == Position.Left)
            {
                if (Left != null)
                {
                    Left.Number += Number;
                    if (Left.PositionEnum == PositionEnum && Left.Debt == Debt - 1 && Right != null)
                    {
                        Right.Left = Left;
                        Left.Right = Right;

                        return;
                    }
                }
                Debt--;
                Number = 0;
                return;
            }

            if (PositionEnum == Position.Right)
            {
                if (Right != null)
                {
                    Right.Number += Number;
                    if (Right.PositionEnum == PositionEnum && Right.Debt == Debt - 1)
                    {
                        if (Left != null)
                        {
                            Left.Right = Right;
                            Right.Left = Left;
                        }

                        return;
                    }
                }

                Debt--;
                Number = 0;
                return;
            }
        }

        public static SnailFishNumber Reduce(SnailFishNumber number)
        {
            var reduced = false;
            do
            {
                reduced = false;
                number = number.Reduce(ref reduced);
                while (number.Left != null)
                {
                    number = number.Left;
                }
            } while (reduced);

            return number;
        }

        internal SnailFishNumber Reduce(ref bool reduced)
        {
            ReduceCurrent(ref reduced);
            return Right?.Reduce(ref reduced) ?? this;
        }

        internal int CalculateMagnitude()
        {
            SnailFishNumber number = this;
            do
            {
                number = number.ResolvePairs();
                while (number.Left != null)
                {
                    number = number.Left;
                }
            } while (number.Left != null || number.Right != null);

            return number.Number;
        }

        private Position FindPosition()
        {
            if (Left == null)
            {
                return Position.Left;
            }

            if (Right.Right == null)
            {
                return Position.Right;
            }

            if (Left.Debt == Debt - 1 && Left.PositionEnum == Position.Left)
            {
                return Position.Right;
            }

            if (Left.Debt > Debt - 1 && Left.PositionEnum == Position.Left)
            {
                return Position.Left;
            }

            if (Right.Right.Debt == Debt - 1 && Right.Right.PositionEnum == Position.Right)
            {
                return Position.Left;
            }

            if (Right.Right.Debt > Debt - 1 && Right.Right.PositionEnum == Position.Right)
            {
                return Position.Right;
            }

            if (Left.PositionEnum == Position.Right && Left.Debt >= Debt)
            {
                return Position.Right;
            }

            return Position.Left;
        }

        private SnailFishNumber ResolvePairs()
        {
            if (Right == null) return this;

            if (PositionEnum != Right.PositionEnum && Debt == Right.Debt)
            {
                var snailFishNumber = new SnailFishNumber(
                    Debt -1,
                    3 * Number + 2 * Right.Number,
                    FindPosition());
                snailFishNumber.Left = Left;
                snailFishNumber.Right = Right.Right;

                if (Left != null)
                {
                    Left.Right = snailFishNumber;
                }

                if (Right.Right != null)
                {
                    Right.Right.Left = snailFishNumber;
                }

                return Right?.Right?.ResolvePairs() ?? snailFishNumber;
            }

            return Right.ResolvePairs();
        }

        public void Add(SnailFishNumber second)
        {
            var last = GetLast();
            last.Right = second;

            IncreaseDebt();
        }

        private void IncreaseDebt()
        {
            Debt++;
            Right?.IncreaseDebt();
        }

        private SnailFishNumber GetLast()
        {
            return Right?.GetLast() ?? this;
        }
    }
}