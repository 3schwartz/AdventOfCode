﻿using Common;
using System.Linq;
using Xunit;

namespace Day18.Tests
{
    public class PairTests
    {
        [Fact]
        public void WhenParse_ThenCorrectPairs()
        {
            // Arrange
            var input = "[[[[[9,8],1],2],3],4]";

            // Act
            var pair = new Pair(input);

            // Assert
            Assert.Equal(4, pair.RightNumber);
            Assert.Null(pair.RightPair);
        }

        [Theory]
        [InlineData("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]")]
        [InlineData("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]")]
        [InlineData("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]")]
        [InlineData("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")]
        [InlineData("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")]
        public void WhenExplode_ThenCorrectPairsAfter(
            string input, string expectedString)
        {
            // Arrange
            var pair = new Pair(input);
            var expected = new Pair(expectedString);

            // Act
            pair.Reduce();

            // Assert
            Assert.Equal(expected, pair);
        }

        [Fact]
        public void WhenSplit_ThenCorrect()
        {
            // Arrange
            var pair = new Pair("[[[[0,7],4],[15,[0,13]]],[1,1]]");
            var expected = new Pair("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

            // Act
            pair.Reduce();

            // Assert
            Assert.Equal(expected, pair);
        }

        [Theory]
        [InlineData("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")]
        [InlineData("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")]
        [InlineData("[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")]
        public void WhenReduce_ThenCorrect(
            string input, string expectedString)
        {
            // Arrange
            var pair = new Pair(input);
            var expected = new Pair(expectedString);

            // Act
            pair.Reduce();

            // Assert
            Assert.Equal(expected, pair);
        }

        [Theory]
        [InlineData("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]", "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")]
        [InlineData("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]", "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]")]
        [InlineData("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]", "[7,[5,[[3,8],[1,4]]]]", "[[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]"
        )]
        public void WhenAdd_ThenConstructCorrect(
            string first, string second, string expectedString)
        {
            // Arrange
            var firstPair = new Pair(first);
            var secondPair = new Pair(second);
            var expected = new Pair(expectedString);

            // Act
            var pair = firstPair.Add(secondPair);

            // Assert
            Assert.Equal(expected, pair);
        }

        [Theory]
        [InlineData("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")]
        [InlineData("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
                )]
        [InlineData(
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
        )]
        [InlineData(
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"
        )]
        [InlineData(
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"
        )]
        public void WhenAdd_ThenReduceCorrect(
            string first, string second, string expectedString)
        {
            // Arrange
            var firstPair = new Pair(first);
            var secondPair = new Pair(second);
            var expected = new Pair(expectedString);
            var pair = firstPair.Add(secondPair);

            // Act
            pair.Reduce();

            // Assert
            Assert.Equal(expected, pair);
        }

        [Theory]
        [InlineData("[[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]",
                    "[[[[0,[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]")]
        [InlineData("[[[[0,[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]",
                    "[[[[7,0],[[14,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]")]
        [InlineData("[[[[7,0],[[14,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]",
                    "[[[[7,14],[0,[14,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]")]
        [InlineData("[[[[7,14],[0,[14,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]",
                    "[[[[7,14],[14,0]],[[[15,7],[8,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]")]
        [InlineData("[[[[7,14],[14,0]],[[[15,7],[8,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]",
                    "[[[[7,14],[14,15]],[[0,[15,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]")]
        [InlineData("[[[[7,14],[14,15]],[[0,[15,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]",
                    "[[[[7,14],[14,15]],[[15,0],[[15,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]")]
        [InlineData("[[[[7,14],[14,15]],[[15,0],[[15,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]",
                    "[[[[7,14],[14,15]],[[15,15],[0,[15,7]]]],[7,[5,[[3,8],[1,4]]]]]")]
        [InlineData("[[[[7,14],[14,15]],[[15,15],[0,[15,7]]]],[7,[5,[[3,8],[1,4]]]]]",
                    "[[[[7,14],[14,15]],[[15,15],[15,0]]],[14,[5,[[3,8],[1,4]]]]]")]
        [InlineData("[[[[7,14],[14,15]],[[15,15],[15,0]]],[14,[5,[[3,8],[1,4]]]]]",
                    "[[[[7,14],[14,15]],[[15,15],[15,0]]],[14,[8,[0,[9,4]]]]]")]
        [InlineData("[[[[7,14],[14,15]],[[15,15],[15,0]]],[14,[8,[0,[9,4]]]]]",
                    "[[[[7,14],[14,15]],[[15,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,14],[14,15]],[[15,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[7,[7,7]],[14,15]],[[15,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,[7,7]],[14,15]],[[15,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[14,0],[21,15]],[[15,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[14,0],[21,15]],[[15,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[[7,7],0],[21,15]],[[15,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[[7,7],0],[21,15]],[[15,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[21,15]],[[15,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[21,15]],[[15,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[[10,11],15]],[[15,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[[10,11],15]],[[15,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,17],[0,26]],[[15,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,17],[0,26]],[[15,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,[8,9]],[0,26]],[[15,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,[8,9]],[0,26]],[[15,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[8,0],[9,26]],[[15,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[8,0],[9,26]],[[15,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[8,0],[9,[13,13]]],[[15,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[8,0],[9,[13,13]]],[[15,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[8,0],[22,0]],[[28,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[8,0],[22,0]],[[28,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[8,0],[[11,11],0]],[[28,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[8,0],[[11,11],0]],[[28,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[8,11],[0,11]],[[28,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[8,11],[0,11]],[[28,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[8,[5,6]],[0,11]],[[28,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[8,[5,6]],[0,11]],[[28,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[13,0],[6,11]],[[28,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[13,0],[6,11]],[[28,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[[6,7],0],[6,11]],[[28,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[[6,7],0],[6,11]],[[28,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[6,11]],[[28,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[6,11]],[[28,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[6,[5,6]]],[[28,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[6,[5,6]]],[[28,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[11,0]],[[34,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[11,0]],[[34,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[[5,6],0]],[[34,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[[5,6],0]],[[34,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,12],[0,6]],[[34,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,12],[0,6]],[[34,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,[6,6]],[0,6]],[[34,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,[6,6]],[0,6]],[[34,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,0],[6,6]],[[34,15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,0],[6,6]],[[34,15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,0],[6,6]],[[[17,17],15],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,0],[6,6]],[[[17,17],15],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,0],[6,23]],[[0,32],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,0],[6,23]],[[0,32],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,0],[6,[11,12]]],[[0,32],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,0],[6,[11,12]]],[[0,32],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,0],[17,0]],[[12,32],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,0],[17,0]],[[12,32],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,0],[[8,9],0]],[[12,32],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,0],[[8,9],0]],[[12,32],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,8],[0,9]],[[12,32],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,8],[0,9]],[[12,32],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,8],[0,9]],[[[6,6],32],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,8],[0,9]],[[[6,6],32],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,8],[0,15]],[[0,38],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,8],[0,15]],[[0,38],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,8],[0,[7,8]]],[[0,38],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,8],[0,[7,8]]],[[0,38],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,8],[7,0]],[[8,38],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,8],[7,0]],[[8,38],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,8],[7,0]],[[8,[19,19]],[15,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,8],[7,0]],[[8,[19,19]],[15,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,8],[7,0]],[[27,0],[34,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,8],[7,0]],[[27,0],[34,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,8],[7,0]],[[[13,14],0],[34,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,8],[7,0]],[[[13,14],0],[34,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,8],[7,13]],[[0,14],[34,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,8],[7,13]],[[0,14],[34,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,8],[7,[6,7]]],[[0,14],[34,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,8],[7,[6,7]]],[[0,14],[34,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,8],[13,0]],[[7,14],[34,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,8],[13,0]],[[7,14],[34,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,8],[[6,7],0]],[[7,14],[34,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,8],[[6,7],0]],[[7,14],[34,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,14],[0,7]],[[7,14],[34,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,14],[0,7]],[[7,14],[34,0]]],[14,[8,[9,0]]]]",
                    "[[[[6,[7,7]],[0,7]],[[7,14],[34,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[6,[7,7]],[0,7]],[[7,14],[34,0]]],[14,[8,[9,0]]]]",
                    "[[[[13,0],[7,7]],[[7,14],[34,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[13,0],[7,7]],[[7,14],[34,0]]],[14,[8,[9,0]]]]",
                    "[[[[[6,7],0],[7,7]],[[7,14],[34,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[[6,7],0],[7,7]],[[7,14],[34,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[7,7]],[[7,14],[34,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[7,7]],[[7,14],[34,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[7,7]],[[7,[7,7]],[34,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[7,7]],[[7,[7,7]],[34,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[7,7]],[[14,0],[41,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[7,7]],[[14,0],[41,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[7,7]],[[[7,7],0],[41,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[7,7]],[[[7,7],0],[41,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[7,14]],[[0,7],[41,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[7,14]],[[0,7],[41,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[7,[7,7]]],[[0,7],[41,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[7,[7,7]]],[[0,7],[41,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[14,0]],[[7,7],[41,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[14,0]],[[7,7],[41,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,7],[[7,7],0]],[[7,7],[41,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,7],[[7,7],0]],[[7,7],[41,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,14],[0,7]],[[7,7],[41,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,14],[0,7]],[[7,7],[41,0]]],[14,[8,[9,0]]]]",
                    "[[[[0,[7,7]],[0,7]],[[7,7],[41,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[0,[7,7]],[0,7]],[[7,7],[41,0]]],[14,[8,[9,0]]]]",
                    "[[[[7,0],[7,7]],[[7,7],[41,0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,0],[7,7]],[[7,7],[41,0]]],[14,[8,[9,0]]]]",
                    "[[[[7,0],[7,7]],[[7,7],[[20,21],0]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,0],[7,7]],[[7,7],[[20,21],0]]],[14,[8,[9,0]]]]",
                    "[[[[7,0],[7,7]],[[7,27],[0,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,0],[7,7]],[[7,27],[0,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,0],[7,7]],[[7,[13,14]],[0,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,0],[7,7]],[[7,[13,14]],[0,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,0],[7,7]],[[20,0],[14,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,0],[7,7]],[[20,0],[14,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,0],[7,7]],[[[10,10],0],[14,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,0],[7,7]],[[[10,10],0],[14,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,0],[7,17]],[[0,10],[14,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,0],[7,17]],[[0,10],[14,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,0],[7,[8,9]]],[[0,10],[14,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,0],[7,[8,9]]],[[0,10],[14,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,0],[15,0]],[[9,10],[14,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,0],[15,0]],[[9,10],[14,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,0],[[7,8],0]],[[9,10],[14,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,0],[[7,8],0]],[[9,10],[14,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[0,8]],[[9,10],[14,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[0,8]],[[9,10],[14,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[0,8]],[[9,[5,5]],[14,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[0,8]],[[9,[5,5]],[14,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[0,8]],[[14,0],[19,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[0,8]],[[14,0],[19,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[0,8]],[[[7,7],0],[19,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[0,8]],[[[7,7],0],[19,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[0,15]],[[0,7],[19,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[0,15]],[[0,7],[19,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[0,[7,8]]],[[0,7],[19,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[0,[7,8]]],[[0,7],[19,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[7,0]],[[8,7],[19,21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,0]],[[8,7],[19,21]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[7,0]],[[8,7],[[9,10],21]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,0]],[[8,7],[[9,10],21]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[7,0]],[[8,16],[0,31]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,0]],[[8,16],[0,31]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[7,0]],[[8,[8,8]],[0,31]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,0]],[[8,[8,8]],[0,31]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[7,0]],[[16,0],[8,31]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,0]],[[16,0],[8,31]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[7,0]],[[[8,8],0],[8,31]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,0]],[[[8,8],0],[8,31]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[0,8],[8,31]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,8]],[[0,8],[8,31]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[0,8],[8,[15,16]]]],[14,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,8]],[[0,8],[8,[15,16]]]],[14,[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[0,8],[23,0]]],[30,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,8]],[[0,8],[23,0]]],[30,[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[0,8],[[11,12],0]]],[30,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,8]],[[0,8],[[11,12],0]]],[30,[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[0,19],[0,12]]],[30,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,8]],[[0,19],[0,12]]],[30,[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[0,[9,10]],[0,12]]],[30,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,8]],[[0,[9,10]],[0,12]]],[30,[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[9,0],[10,12]]],[30,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,8]],[[9,0],[10,12]]],[30,[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[9,0],[[5,5],12]]],[30,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,8]],[[9,0],[[5,5],12]]],[30,[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[9,5],[0,17]]],[30,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,8]],[[9,5],[0,17]]],[30,[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[9,5],[0,[8,9]]]],[30,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,8]],[[9,5],[0,[8,9]]]],[30,[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[9,5],[8,0]]],[39,[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,8]],[[9,5],[8,0]]],[39,[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[9,5],[8,0]]],[[19,20],[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,8]],[[9,5],[8,0]]],[[19,20],[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[9,5],[8,0]]],[[[9,10],20],[8,[9,0]]]]")]
        [InlineData("[[[[7,7],[7,8]],[[9,5],[8,0]]],[[[9,10],20],[8,[9,0]]]]",
                    "[[[[7,7],[7,8]],[[9,5],[8,0]]],[[[9,[5,5]],20],[8,[9,0]]]]")]
        public void WhenReduceCurrentSecond_ThenCorrect(string input, string expectedString)
        {
            // Arrange
            var pair = new Pair(input);
            var expected = new Pair(expectedString);

            // Act
            pair.ReduceCurrent();

            // Assert
            Assert.Equal(expected, pair);
        }

        [Theory]
        [InlineData("[[9,1],[1,9]]", 129)]
        [InlineData("[[1,2],[[3,4],5]]", 143)]
        [InlineData("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)]
        [InlineData("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)]
        [InlineData("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)]
        [InlineData("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)]
        [InlineData("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488)]
        public void WhenCalculateMagnitude_ThenCorrect(string input, int magnitude)
        {
            // Arrange
            var pair = new Pair(input);

            // Act
            var actual = pair.CalculateMagnitude();

            // Assert
            Assert.Equal(magnitude, actual);
        }

        [Fact]
        public void WhenCalculateHomework_ThenCorrectSum()
        {
            // Arrange
            var numbers = DataLoader.GetData("../../../../../data/day18_data_test.txt")
                .Select(d => new Pair(d)).ToList();

            // Act
            var actual = Pair.CalculateHomework(numbers);

            // Assert
            Assert.Equal(4140, actual);
        }

        [Fact]
        public void WhenCalculateMaxMagnitude_ThenCorrect()
        {
            // Arrange
            var numbers = DataLoader.GetData("../../../../../data/day18_data_test.txt");

            // Act
            var actual = Pair.CalculateMaxMagnitude(numbers);

            // Assert
            Assert.Equal(3993, actual);
        }
    }
}
