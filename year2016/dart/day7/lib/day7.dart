bool isTls(String input) {
  var isOutside = true;
  var hasOutside = false;
  var hasInside = false;
  for (int i = 0; i < input.length - 3; i++) {
    if (input[i] == '[') {
      isOutside = false;
      continue;
    }
    if (input[i] == ']') {
      isOutside = true;
      continue;
    }
    if (input[i] == input[i + 3] &&
        input[i + 1] == input[i + 2] &&
        input[i] != input[i + 1]) {
      
      if (isOutside) {
        hasOutside = true;
      } else {
        hasInside = true;
      }
    }
  }
  return !hasInside && hasOutside;
}

bool isSsl(String input) {
  var isOutside = true;
  Set<String> inside = {};
  Set<String> outside = {};
  for (int i = 0; i < input.length - 2; i++) {
    switch (input[i]) {
      case '[':
        isOutside = false;
        break;
      case ']':
        isOutside = true;
        break;
      default:
        if (input[i] == input[i+2] && input[i] != input[i+1]) {
          if (isOutside) {
            outside.add(input.substring(i, i+3));
          } else {
            inside.add("${input[i+1]}${input[i]}${input[i+1]}");
          }
        }
        break;
    }
  }

  return inside.intersection(outside).isNotEmpty;
}