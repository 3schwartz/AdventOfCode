import 'dart:io';

import 'package:day2/day2.dart';
import 'package:test/test.dart';

Future<void> main() async {
  var lines = await File("../../data/day2_data_test.txt").readAsLines();
  
  test('part1', () {
    // Act
    var code = findCode(lines, Coord(1, 1));

    // Assert
    expect(code, "1985");
  });

  test('part2', () {
    // Act
    var code = findCode(lines, FancyCoord(0, 2));

    // Assert
    expect(code, "5DB3");
  });
}
