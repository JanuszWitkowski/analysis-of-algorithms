def proper_name (filename):
    name = filename[:-4]
    while name[0] == '.' or name[0] == '/':
        name = name[1:]
    return name

def open_numbers(filename):
    with open(filename) as file:
        numbers = [int(line.rstrip()) for line in file]
    return numbers
