import 'dart:io';

Future<void> main(List<String> arguments) async {
  var lines = await File("../../data/day3_data.txt").readAsLines();

  var valids = 0;
  for (var line in lines) {
    var parts = createParts(line);
    if (isValid(parts)) {
      valids++;
    }
  }

  print('Part 1: $valids');

  var flattened = zipAndFlatten(lines);
  var idx = flattened.length ~/ 3;
  var validsColumns = 0;
  for (int i = 0; i < idx; i++) {
    var from = i * 3;
    var to = (i + 1) * 3;
    if (isValid(flattened.sublist(from, to))) {
      validsColumns++;
    }
  }

  print("Part 2: $validsColumns");
}

List<int> zipAndFlatten(List<String> lines) {
  var partList = [];
  for (var line in lines) {
    var parts = createParts(line);  
    partList.add(parts);
  }
  List<List<int>> columns = [<int>[], <int>[], <int>[]];
  for (var parts in partList) {
    columns[0].add(parts[0]);
    columns[1].add(parts[1]);
    columns[2].add(parts[2]);
  }
  var flattened = columns.expand((element) => element).toList();
  return flattened;
}

List<int> createParts(String line) {
  var parts = line.trim().split(RegExp(r'\s+'))
    .map((e) => int.parse(e))
    .toList();
  return parts;
}

bool isValid(List<int> parts) {
  if (parts.length != 3) {
    throw Exception(parts);
  }
  for (var combination in [(0,1,2), (0,2,1), (1,2,0)]) {
    if (parts[combination.$1] + parts[combination.$2] <= parts[combination.$3]) {
      return false;
    }
  }
  return true;
}
