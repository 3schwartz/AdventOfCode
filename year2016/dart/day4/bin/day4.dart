import 'dart:io';

import 'package:day4/day4.dart';


Future<void> main(List<String> arguments) async {
  var lines = await File("../../data/day4_data.txt").readAsLines();

  var part1 = findSectorSum(lines);

  print("Part 1: $part1");

  for (var line in lines) {
    var decrypted = decrypt(line);
    if (decrypted.startsWith("northpole object storage")) {
      print("Part 2:");
      print(decrypted);
    }
  }
}
