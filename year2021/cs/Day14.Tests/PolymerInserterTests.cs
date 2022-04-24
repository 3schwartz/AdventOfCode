using System;
using System.Collections.Generic;
using Xunit;
using static Day14.PolymerInserter;

namespace Day14.Tests;

public class PolymerInserterTests
{
    private readonly PolymerInserter polymerInserter;

    public PolymerInserterTests()
    {
        polymerInserter = new PolymerInserter();
    }

    [Theory]
    [InlineData(10, 1588)]
    public void WhenUsingTestData_ThenCorrect(int insertions, long expected)
    {
        // Arrange
        var data = DataLoader.GetData("../../../../../data/day14_data_test.txt");

        // Act
        var polymerTemplate = polymerInserter.DoInsertion(data, insertions);
        int mostCommonMinusLeastCommon = polymerInserter.GetMostCommonMinusLeastCommon(polymerTemplate);

        // Act
        Assert.Equal(expected, mostCommonMinusLeastCommon);
    }

    [Theory]
    [InlineData("NCNBCHB", 1)]
    [InlineData("NBCCNBBBCBHCB", 2)]
    [InlineData("NBBBCNCCNBBNBNBBCHBHHBCHB", 3)]
    [InlineData("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB", 4)]
    public void WhenDoingInsertion_ThenReturnUpdatedTemplate(string expected, int iterations)
    {
        // Arrange
        var data = DataLoader.GetData("../../../../../data/day14_data_test.txt");

        // Act
        var polymerTemplate = polymerInserter.DoInsertion(data, iterations);

        // Act
        Assert.Equal(expected.AsSpan().ToArray(), polymerTemplate.ToArray());
    }

    [Fact]
    public void WhenGivenLineRules_ThenCreateRules()
    {
        // Arrange
        var lineRules = new string[2]{ "HB -> C", "HC -> B" };

        // Act
        IList<Rule> rules = polymerInserter.CreateRules(lineRules);

        // Assert
        Assert.Equal(2, rules.Count);
        Assert.Equal('H', rules[0].First);
        Assert.Equal('B', rules[0].Second);
        Assert.Equal('C', rules[0].Insert);
    }

    [Fact]
    public void GivenRules_WhenDoCycle_ThenInsert()
    {
        // Arrange
        var expected = new char[7] { 'N', 'Q', 'H', 'F', 'H', 'C', 'F' };
        Span<char> template = new char[5] { 'N', 'H', 'H', 'C', 'F' };
        var rules = new List<Rule> { new Rule('H', 'H', 'F'), new Rule('N', 'H', 'Q')};

        // Act
        Span<char> actual = polymerInserter.DoCycle(template, rules);

        // Assert
        Assert.Equal(expected, actual.ToArray());
    }

    [Fact]
    public void WhenGivenRules_ThenFindRulesToApply()
    {
        // Arrange
        Span<char> template = new char[5] { 'N', 'H', 'H', 'C', 'F' };
        var rules = new List<Rule> { new Rule('H', 'H', 'F'), new Rule('N', 'H', 'Q') };
        var expected = new List<RulesToApply>{ 
            new RulesToApply(1, 'Q'), new RulesToApply(2, 'F') 
        };

        // Act
        var actual = polymerInserter.FindRulesToApply(template, rules);

        // Assert
        Assert.Equal(expected, actual);
    }

    [Fact]
    public void WhenGivenRulesFound_ThenApply()
    {
        // Arrange
        var expected = new char[7] { 'N', 'Q', 'H', 'F', 'H', 'C', 'F' };
        Span<char> template = new char[5] { 'N', 'H', 'H', 'C', 'F' };
        var foundRules = new List<RulesToApply>{
            new RulesToApply(1, 'Q'), new RulesToApply(2, 'F')
        };

        // Act
        Span<char> actual = polymerInserter.ApplyFoundRules(template, foundRules);

        // Assert
        Assert.Equal(expected, actual.ToArray());
    }
}