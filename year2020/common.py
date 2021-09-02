from pathlib import Path

def get_lines(path):
    lines = []

    fn = Path(__file__).parent / path

    with open(fn) as f:
        for line in f:
            lines.append(line.rstrip('\n'))
    
    return lines

