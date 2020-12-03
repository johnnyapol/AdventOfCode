#!/usr/bin/env python3

def get_slope_amount(slope_y, slope_x, input):
    answer = 0
    pos_x = 0
    pos_y = 0
    while True:
        pos_x += slope_x
        pos_y += slope_y

        if (pos_x >= len(input)):
            break
        slot = input[pos_x][pos_y % (len(input[0]) - 1)]

        if slot == '#':
            answer += 1
    
    return answer

def main():
    input = open("input.txt", 'r').readlines()
    print(f"Part 1: {get_slope_amount(3,1,input)}")

    slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)]

    ans = 1

    for y,x in slopes:
        ans *= get_slope_amount(y,x,input)

    print(f"Part 2: {ans}")


if __name__ == "__main__":
    main()