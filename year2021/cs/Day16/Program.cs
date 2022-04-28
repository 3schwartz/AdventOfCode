using Day16;

var hexString = File.ReadAllText("../../../../../data/day16_data.txt");
var binary = HexConverter.ToBinary(hexString);

var packet = new Packet(ref binary);
var totalVersion = packet.GetTotalVersion();

Console.WriteLine($"Part 1: {totalVersion}");

var evaluation = packet.Evaluate();

Console.WriteLine($"Part 2: {evaluation}");