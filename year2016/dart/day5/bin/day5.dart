
import 'package:day5/day5.dart';

void main(List<String> arguments) {
  final doorId = "cxdnnyjw";

  final part1 = getPassword(doorId) ;

  print("Part 1: $part1");

  final part2 = getComplexPassword(doorId);

  print("Part 2: $part2");
}


