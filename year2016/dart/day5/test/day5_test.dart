import 'package:day5/day5.dart';
import 'package:test/test.dart';

void main() {

  test('part2', () {
    // Arrange
    final id = "abc";

    // Act
    final password = getComplexPassword(id);

    // Assert
    expect(password, "05ace8e3");
  });

  test('part2', () {
    // Arrange
    final id = "abc";

    // Act
    final password = getPassword(id);

    // Assert
    expect(password, "18f47a30");
  });

  test('hash', () {
    // Arrange
    final id = "abc";
    final idx = 5017308;

    // Act
    var hash = getHash(id, idx);

    // Assert
    expect(hash.substring(0,9), "000008f82");
  });
}
