import 'dart:io';

import 'package:day6/day6.dart';

Future<void> main(List<String> arguments) async {
  final lines = await File('../../data/day6_data.txt').readAsLines();

  final messageLow = findMessage(lines, highest);

  print("Part 1: $messageLow");

  final messageHigh = findMessage(lines, lowest);

  print("Part 2: $messageHigh");
}
