
String findMessage(
  List<String> lines,
  MapEntry<int,int> Function(MapEntry<int, int>, MapEntry<int, int>) reduction) 
  {
  final storage = createIndexCounts(lines);
  return createStringFromCounts(storage, reduction);
}

String createStringFromCounts(
  Map<int,Map<int,int>> storage,
  MapEntry<int,int> Function(MapEntry<int, int>, MapEntry<int, int>) reduction) {
  var maxIdx = storage.entries.reduce((a, b) => a.key > b.key ? a : b);
  List<int> codes = [];
  for (var i = 0; i <= maxIdx.key; i++) {
    final map = storage[i]!;

    var maxEntry = map.entries.reduce(reduction);
    codes.add(maxEntry.key);
  }
  return String.fromCharCodes(codes);
}

Map<int,Map<int,int>> createIndexCounts(List<String> lines) {
  Map<int,Map<int,int>> storage = {};
  for (var line in lines) {
    for (var (idx, rune) in line.runes.enumerate()) {
      final map = storage[idx] ?? {};
      map[rune] = (map[rune] ?? 0) + 1;
      storage[idx] = map;
    }
  }
  return storage;
}

MapEntry<int,int> Function(MapEntry<int, int>, MapEntry<int, int>) highest = (a, b) => a.value > b.value ? a : b;
MapEntry<int,int> Function(MapEntry<int, int>, MapEntry<int, int>) lowest = (a, b) => a.value < b.value ? a : b;

extension Enumerate<T> on Iterable<T> {
  Iterable<(int, T)> enumerate() sync* {
    int index = 0;
    for (final element in this) {
      yield (index, element);
      index++;
    }
  }
}
