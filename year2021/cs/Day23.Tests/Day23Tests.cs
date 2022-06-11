using System.Collections.Generic;
using System.Linq;
using Xunit;

namespace Day23.Tests;

public class Day23Tests
{

    [Fact]
    public void WhenFindEnd_ThenCorrect()
    {
        // Arrange & Act
        var end = AmphipodStringSorter.GetEnd(2);

        // Assert
        Assert.Equal("...........AABBCCDD", end);
    }

    [Fact]
    public void GivenInputRooms_WhenCreateInitialState_ThenCorrect()
    {
        // Arrange
        var rooms = new List<char>()
            {'B', 'A', 'C', 'D', 'B', 'C', 'D', 'A'};
        var roomSize = 2;

        // Act
        var inital = AmphipodStringSorter.GetInitial(rooms, roomSize);

        // Assert
        Assert.Equal("...........BACDBCDA", inital);
    }

    [Theory]
    [InlineData(0, "BA")]
    [InlineData(1, "CD")]
    [InlineData(2, "BC")]
    [InlineData(3, "DA")]
    public void GivenRoom_WhenGet_ThenCorrect(int idx, string room)
    {
        // Arrange
        var state = "...........BACDBCDA";
        var roomSize = 2;
        var sorter = new AmphipodStringSorter(roomSize);

        // Act
        var roomSpan = sorter.GetRoom(idx, state);

        // Assert
        Assert.Equal(room, roomSpan.ToString());
    }

    [Theory]
    [InlineData('A', 1)]
    [InlineData('B', 2)]
    public void WhenCompareState_ThenOnlyIncreaseSetWhenDifferent(char member, int size)
    {
        // Arrang
        var roomsOne = new List<Stack<char>> {
            CreateRoom('B', 'A'),
            CreateRoom('C', 'D'),
            CreateRoom('B', 'C'),
            CreateRoom('D', member)};
        var roomsSecond = new List<Stack<char>> {
            CreateRoom('B', 'A'),
            CreateRoom('C', 'D'),
            CreateRoom('B', 'C'),
            CreateRoom('D', 'A')};
        var stateOne = new AmphipodRoomSorter.State(Enumerable.Repeat('.', 11).ToList(), roomsOne);
        var stateSecond = new AmphipodRoomSorter.State(Enumerable.Repeat('.', 11).ToList(), roomsSecond);
        var visited = new HashSet<AmphipodRoomSorter.State>();

        // Act
        visited.Add(stateOne);
        visited.Add(stateSecond);

        // Assert
        Assert.Equal(size, visited.Count);
    }

    [Theory]
    [InlineData("...........BACDBCDA", 0, "...B.......BACD.CDA", 40)]
    [InlineData("...B.......BACD.CDA", 40, "...B.C.....BA.D.CDA", 240)]
    [InlineData("...B.C.....BA.D.CDA", 240, "...B.......BA.DCCDA", 440)]
    public void GivenState_WhenFindGetPossible_ThenFindAtLeastOptimal(
        string state,
        int cost,
        string expectedState,
        int expectedCost)
    {
        // Arrange
        var rooms = new List<char>()
            {'B', 'A', 'C', 'D', 'B', 'C', 'D', 'A'};
        var roomSize = 2;
        var sorter = new AmphipodStringSorter(roomSize);

        // Act

        var states = sorter.GetPossibleStates(state, cost);

        // Assert
        var actualState = states.First(s => s.State == expectedState);
        Assert.NotNull(actualState);
        Assert.Equal(expectedCost, actualState.Cost);

    }

    [Fact]
    public void GivenStringSorterSmallRoom_WhenCalculateLeastEnergy_ThenCorrect()
    {
        // Arrange
        var rooms = new List<char>()
            {'B', 'A', 'C', 'D', 'B', 'C', 'D', 'A'};
        var roomSize = 2;
        var sorter = new AmphipodStringSorter(roomSize);

        // Act
        var energy = sorter.CalculateLeastEnergy(rooms);

        // Assert
        Assert.Equal(12521, energy);
    }

    [Fact]
    public void GivenStringSorterBigRoom_WhenCalculateLeastEnergy_ThenCorrect()
    {
        // Arrange
        var rooms = new List<char>()
            {'B', 'D','D', 'A', 'C','C','B', 'D', 'B','B','A', 'C', 'D','A','C', 'A'};
        var roomSize = 4;
        var sorter = new AmphipodStringSorter(roomSize);

        // Act
        var energy = sorter.CalculateLeastEnergy(rooms);

        // Assert
        Assert.Equal(44169, energy);
    }

    [Fact(Skip = "Really slow")]
    public void GivenRooms_WhenCalculateLeastEnergy_ThenCorrect()
    {
        // Arrange
        var rooms = new List<Stack<char>> {
            CreateRoom('B', 'A'),
            CreateRoom('C', 'D'),
            CreateRoom('B', 'C'),
            CreateRoom('D', 'A')};
        var sorter = new AmphipodRoomSorter();

        // Act
        var energy = sorter.CalculateLeastEnergy(rooms);

        // Assert
        Assert.Equal(12521, energy);
    }

    private Stack<char> CreateRoom(char outer, char inner)
    {
        var stack = new Stack<char>();
        stack.Push(inner);
        stack.Push(outer);
        return stack;
    }
    
    
}