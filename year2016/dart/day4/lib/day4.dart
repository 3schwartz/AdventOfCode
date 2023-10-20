
String decrypt(String line) {
  var sectorIdPattern = RegExp(r'(\d+)');
  var sectorId = int.parse((sectorIdPattern.firstMatch(line)?.group(1))!);
  var code = StringBuffer();
  for (var rune in line.runes) {
    if (rune == 45) { // -
        code.write(" ");
        continue;
    }
    if (rune < 97) { // a
        break;
    }
    var shifted = ((rune - 97) + sectorId) % 26 + 97;
    var shiftedCode = String.fromCharCode(shifted);
    code.write(shiftedCode);
  }
  code.write("- $sectorId");
  return code.toString();
}

int findSectorSum(List<String> lines) {
  var sectorSum = 0;
  var pattern = RegExp(r'\[(.*?)\]');
  var sectorIdPattern = RegExp(r'(\d+)');
  for (var line in lines) {
    var checksum = (pattern.firstMatch(line)?.group(1))!;

    Map<int,int> storage = {};
    for (var rune in line.runes) {
      if (rune == 45) { // -
        continue;
      }
      if (rune < 97) { // a
        break;
      }
      
      storage[rune] = (storage[rune] ?? 0) + 1;
    }
    var entries = storage.entries.toList();
    entries.sort((a, b) {
      var comparision = b.value.compareTo(a.value);
      if (comparision != 0) {
        return comparision;
      }
      return a.key.compareTo(b.key);
    });
    var builder = StringBuffer();
    for (var i = 0; i < 5; i++) {
      var code = entries[i].key;
      var char = String.fromCharCode(code);
      builder.write(char);
    }
    var found = builder.toString();

    if (found != checksum) {
      continue;
    }

    sectorSum += int.parse((sectorIdPattern.firstMatch(line)?.group(1))!);
  }
  return sectorSum;  
}
