import 'dart:io';

import 'package:day6/day6.dart';
import 'package:test/test.dart';

void main() async {
  final lines = await File('../../data/day6_data_test.txt').readAsLines();

  test('part1', () {
    // Act
    final message = findMessage(lines, highest);

    // Assert
    expect(message, "easter");
  });

  test('part2', () {
    // Act
    final message = findMessage(lines, lowest);

    // Assert
    expect(message, "advent");
  });
}
