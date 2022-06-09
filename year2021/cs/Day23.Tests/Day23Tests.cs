using System;
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
        var rooms = new List<char>()
            {'B', 'A', 'C', 'D', 'B', 'C', 'D', 'A'};
        var state = "...........BACDBCDA";
        var roomSize = 2;
        var sorter = new AmphipodStringSorter(rooms, roomSize);

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

    [Fact]
    public void GivenStringSorter_WhenCalculateLeastEnergy_ThenCorrect()
    {
        // Arrange
        var rooms = new List<char>()
            {'B', 'A', 'C', 'D', 'B', 'C', 'D', 'A'};
        var roomSize = 2;
        var sorter = new AmphipodStringSorter(rooms, roomSize);

        // Act
        var energy = sorter.CalculateLeastEnergy();

        // Assert
        Assert.Equal(12521, energy);
    }

    //[Fact]
    //public void GivenRooms_WhenCalculateLeastEnergy_ThenCorrect()
    //{
    //    // Arrange
    //    var rooms = new List<Stack<char>> {
    //        CreateRoom('B', 'A'),
    //        CreateRoom('C', 'D'),
    //        CreateRoom('B', 'C'),
    //        CreateRoom('D', 'A')};
    //    var sorter = new AmphipodRoomSorter();

    //    // Act
    //    var energy = sorter.CalculateLeastEnergy(rooms);

    //    // Assert
    //    Assert.Equal(12521, energy);
    //}

    //[Fact]
    //public void GivenRooms_WhenCalculateLeastEnergy_ThenCorrect2()
    //{
    //    // Arrange
    //    var rooms = new List<Stack<char>> {
    //        CreateRoom('D', 'B'),
    //        CreateRoom('A', 'C'),
    //        CreateRoom('D', 'B'),
    //        CreateRoom('C', 'A')};
    //    var sorter = new AmphipodRoomSorter();

    //    // Act
    //    var energy = sorter.CalculateLeastEnergy(rooms);

    //    // Assert
    //    Assert.Equal(12521, energy);
    //}

    private Stack<char> CreateRoom(char outer, char inner)
    {
        var stack = new Stack<char>();
        stack.Push(inner);
        stack.Push(outer);
        return stack;
    }
    
    
}