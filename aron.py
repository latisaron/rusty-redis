ary = "*2\r\n$3\r\nget\r\n$3\r\nkey\r\n"
ary.split("\r\n")

def deserialize(string):
    split_ary = string.split('\r\n')
    if split_ary[0][0] == '+':
        return int(split_ary[0][1:-1])
    elif split_ary[0][0] == '$':
        return split_ary[1][1:-1]
    elif split_ary[0][0] == '*':
        num_elem = int(split_ary[0][1:-1])
        for i in range(num_elem):