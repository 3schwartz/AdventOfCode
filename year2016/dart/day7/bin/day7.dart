import 'dart:io';

import 'package:day7/day7.dart';

Future<void> main(List<String> arguments) async {
  final lines = await File("../../data/day7_data.txt").readAsLines();

  var supportTls = 0;
  var supportSsl = 0;
  for (var line in lines) {
    supportTls += isTls(line) ? 1 : 0;
    supportSsl += isSsl(line) ? 1: 0;
  }

  print("Part 1: $supportTls");
  print("Part 2: $supportSsl");
}
