import 'dart:convert';

import 'package:crypto/crypto.dart';

String getHash(String id, int idx) {
  final concat = id + idx.toString();
  final bytes = utf8.encode(concat);
  final hash = md5.convert(bytes).toString();
  return hash;
}

String getComplexPassword(String id) {
  final vector = List<String>.filled(8, "");
  var i = 0;
  var found = 0;
  while(found != 8) {
    final hash = getHash(id, i);
    i++;
    if ("00000" != hash.substring(0,5)) {
      continue;
    }
    final parsed = int.tryParse(hash[5]);
    if (parsed == null || parsed > 7 || vector[parsed] != "") {
      continue;
    }
    vector[parsed] = hash[6];
    found++;
  }
  return vector.join("");
}

String getPassword(String id) {
  var buffer = StringBuffer();
  var i = 0;
  while(buffer.length != 8) {
    final hash = getHash(id, i);

    if ("00000" == hash.substring(0, 5)) {
      buffer.write(hash[5]);
    }
    i++;
  }
  return buffer.toString();
}