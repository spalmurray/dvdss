#!/usr/bin/env python3


class Pixel:
    value: bool

    def __init__(self, value: bool):
        self.value = value

    def __str__(self):
        return "true" if self.value else "false"

    def __repr__(self):
        return "true" if self.value else "false"


if __name__ == "__main__":
    with open("shape") as shape:
        lines = shape.readlines()
        lines = [line[0:-1] for line in lines]
        width = len(lines[0])
        height = len(lines)
        pixels: list[list[Pixel]] = []
        for i in range(width):
            pixels.append([Pixel(False)] * height)

        for j in range(height):
            line = lines[j]
            for i in range(width):
                pixels[i][j] = Pixel(line[i] == "#")

        print(pixels)
