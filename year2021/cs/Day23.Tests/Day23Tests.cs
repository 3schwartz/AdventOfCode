using System.Collections.Generic;
using System.Linq;
using Xunit;

namespace Day23.Tests;

public class Day23Tests
{
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

    [Fact]
    public void GivenRooms_WhenCalculateLeastEnergy_ThenCorrect2()
    {
        // Arrange
        var rooms = new List<Stack<char>> {
            CreateRoom('D', 'B'),
            CreateRoom('A', 'C'),
            CreateRoom('D', 'B'),
            CreateRoom('C', 'A')};
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