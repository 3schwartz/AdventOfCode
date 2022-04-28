using System;
using Xunit;

namespace Day16.Tests;

public class Day16Tests
{
    [Theory]
    [InlineData("C200B40A82", 3)]
    [InlineData("04005AC33890", 54)]
    [InlineData("880086C3E88112", 7)]
    [InlineData("CE00C43D881120", 9)]
    [InlineData("D8005AC2A8F0", 1)]
    [InlineData("F600BC2D8F", 0)]
    [InlineData("9C005AC2F8F0", 0)]
    [InlineData("9C0141080250320F1802104A08", 1)]
    public void WhenCalculateExpression_ThenCorrectValue(string hexString, long? value)
    {
        // Arrange
        var binary = HexConverter.ToBinary(hexString);
        var packet = new Packet(ref binary);

        // Act
        var expressionValue = packet.Evaluate();

        // Assert
        Assert.Equal(value, expressionValue);
    }

    [Theory]
    [InlineData("D2FE28", "110100101111111000101000")]
    [InlineData("38006F45291200", "00111000000000000110111101000101001010010001001000000000")]
    [InlineData("EE00D40C823060", "11101110000000001101010000001100100000100011000001100000")]
    public void WhenGivenHex_ThenReturnBinary(string hexString, string expected)
    {
        // Arrange & Act
        var binary = HexConverter.ToBinary(hexString);

        // Assert
        Assert.Equal(expected, binary.ToString());
    }

    [Fact]
    public void WhenGivenPacketWithLiteralValue_ThenTranslateToPacket()
    {
        // Arrange
        var packetSpan = "110100101111111000101000".AsSpan();

        // Act
        var packet = new Packet(ref packetSpan);

        // Assert
        Assert.Equal(6, packet.Version);
        Assert.Equal(4, packet.Id);
        Assert.Equal(2021, packet.LiteralValue);
        Assert.Equal(6, packet.GetTotalVersion());
    }

    [Fact]
    [Obsolete("Just for fun to compare memory allocation compared to span")]
    public void GivenStringConstrucotr_WhenGivenPacketWithLiteralValue_ThenTranslateToPacket()
    {
        // Arrange
        var packetSpan = "110100101111111000101000";

        // Act
        var packet = new Packet(ref packetSpan);

        // Assert
        Assert.Equal(6, packet.Version);
        Assert.Equal(4, packet.Id);
        Assert.Equal(2021, packet.LiteralValue);
        Assert.Equal(6, packet.GetTotalVersion());
    }

    [Fact]
    public void GivenLengthTypeId0_WhenDecodeSubPackets_ThenCorrectPacket()
    {
        // Arrange
        var packetSpan = "00111000000000000110111101000101001010010001001000000000".AsSpan();

        // Act
        var packet = new Packet(ref packetSpan);

        Assert.Equal(1, packet.Version);
        Assert.Equal(6, packet.Id);
        Assert.NotNull(packet.Packets);
        Assert.Equal(10, packet.Packets![0].LiteralValue);
        Assert.Equal(20, packet.Packets![1].LiteralValue);
    }

    [Fact]
    public void GivenLengthTypeId1_WhenDecodeSubPackets_ThenCorrectPacket()
    {
        // Arrange
        var packetSpan = "11101110000000001101010000001100100000100011000001100000".AsSpan();

        // Act
        var packet = new Packet(ref packetSpan);

        Assert.Equal(7, packet.Version);
        Assert.Equal(3, packet.Id);
        Assert.NotNull(packet.Packets);
        Assert.Equal(1, packet.Packets![0].LiteralValue);
        Assert.Equal(2, packet.Packets![1].LiteralValue);
        Assert.Equal(3, packet.Packets![2].LiteralValue);
    }
}

