def proper_name (filename):
    name = filename[:-4]
    # while name[0] == '.' or name[0] == '/':
    #     name = name[1:]
    idx = 0
    l = len(name)
    while idx < l and name[-idx] != '/':
        idx += 1
    if idx == l:
        return name
    return name[-idx+1:]

def open_numbers(filename):
    with open(filename) as file:
        numbers = [int(line.rstrip()) for line in file]
    return numbers
