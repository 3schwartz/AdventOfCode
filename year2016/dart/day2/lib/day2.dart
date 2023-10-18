String findCode(List<String> lines, Position current) {
  var code = StringBuffer();
  for (var line in lines) {
    for (var char in line.runes) {
      current = current.move(char);
    }
    code.write(current.print());
  }
  return code.toString();
}

abstract class Position {
  String print();
  Position move(int value);
}

class FancyCoord implements Position {
  final int x;
  final int y;

  static const Map<(int,int), String> lookup = {
    (3,0): "1",
    
    (1,1): "2",
    (2,1): "3",
    (3,1): "4",

    (0,2): "5",
    (1,2): "6",
    (2,2): "7",
    (3,2): "8",
    (4,2): "9",

    (1,3): "A",
    (2,3): "B",
    (3,3): "C",

    (2,4): "D",
  };

  FancyCoord(this.x, this.y);

  @override
  String print() {
    return lookup[(x,y)]!;
  }  

  @override
  FancyCoord move(int value) {
    switch (value) {
      case 85: // U
        if (x == 0 && y == 2 ||
            x == 1 && y == 1 ||
            x == 2 && y == 0 ||
            x == 3 && y == 1 ||
            x == 4 && y == 2) {
              break;
            }
        return FancyCoord(x, y - 1);
      case 68: // D
        if (x == 0 && y == 2 ||
            x == 1 && y == 3 ||
            x == 2 && y == 4 ||
            x == 3 && y == 3 ||
            x == 4 && y == 2) {
              break;
            }
        return FancyCoord(x, y + 1);
      case 82: // R
        if (x == 2 && y == 0 ||
            x == 3 && y == 1 ||
            x == 4 && y == 2 ||
            x == 3 && y == 3 ||
            x == 2 && y == 4) {
              break;
            }
        return FancyCoord(x + 1, y);
      case 76: // L
        if (x == 2 && y == 0 ||
            x == 1 && y == 1 ||
            x == 0 && y == 2 ||
            x == 1 && y == 3 ||
            x == 2 && y == 4) {
              break;
            }
        return FancyCoord(x - 1, y);
      default:
        throw Exception('Not known: $value');    
    }
    return FancyCoord(x, y);
  }
}

class Coord implements Position {
  final int x;
  final int y;

  static const Map<(int,int), int> lookup = {
    (0,0): 1,
    (1,0): 2,
    (2,0): 3,
    (0,1): 4,
    (1,1): 5,
    (2,1): 6,
    (0,2): 7,
    (1,2): 8,
    (2,2): 9
  };

  Coord(this.x, this.y);

  @override
  String print() {
    return lookup[(x,y)].toString();
  }

  @override
  Coord move(int value) {
    switch (value) {
      case 85: // U
        if (y > 0) {
          return Coord(x, y - 1);
        }        
        break;
      case 68: // D
        if (y < 2) {
          return Coord(x, y + 1);
        }
        break;
      case 82: // R
        if (x < 2) {
          return Coord(x + 1, y);
        }
        break;
      case 76: // L
        if (x > 0) {
          return Coord(x - 1, y);
        }
        break;
      default:
        throw Exception('Not known: $value');    
    }
    return Coord(x, y);
  }
}
