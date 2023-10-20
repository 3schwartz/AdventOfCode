import 'dart:io';

import 'package:day4/day4.dart';
import 'package:test/test.dart';

Future<void> main() async {
  var lines = await File("../../data/day4_data_test.txt").readAsLines();

  test('part1', () {
    // Act
    var valid = findSectorSum(lines);

    // Assert
    expect(valid, 1514);
  });

  test('part2', () {
    // Arrange
    final input = "qzmt-zixmtkozy-ivhz-343";
    final expected = "very encrypted name - 343";

    // Act
    var decrypted = decrypt(input);

    // Assert
    expect(decrypted, expected);
  });  
}
