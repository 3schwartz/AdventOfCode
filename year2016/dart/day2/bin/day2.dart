import 'dart:io';

import 'package:day2/day2.dart';

Future<void> main(List<String> arguments) async {
  var lines = await File("../../data/day2_data.txt").readAsLines();

  var code = findCode(lines, Coord(1, 1));

  print("Part 1: $code");

  code = findCode(lines, FancyCoord(0, 2));

  print("Part 1: $code");
}

