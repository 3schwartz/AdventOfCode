import 'package:day7/day7.dart';
import 'package:test/test.dart';

void main() {
  test('tls', () {
    // Arrange
    const input = "abba[mnop]qrst";
    const expected = true;

    // Act
    var actual = isTls(input);

    // Assert
    expect(actual, expected);
  });

  group("tls", () {
    var inputs = [
      ("abba[mnop]qrst", true),
      ("abcd[bddb]xyyx", false),
      ("aaaa[qwer]tyui", false),
      ("ioxxoj[asdfgh]zxcvbn", true)
    ];
    for (var input in inputs) {
      test(input.$1, () {
        expect(input.$2, isTls(input.$1));
      });
    }
  });

  group("tls", () {
    var inputs = [
      ("aba[bab]xyz", true),
      ("xyx[xyx]xyx", false),
      ("aaa[kek]eke", true),
      ("zazbz[bzb]cdb", true)
    ];
    for (var input in inputs) {
      test(input.$1, () {
        expect(input.$2, isSsl(input.$1));
      });
    }
  });
}
