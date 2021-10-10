import re
import functools


class Decoder:
    def __init__(self):
        self.regexp = re.compile(r'(\\x\d{2}|"|\\)')
        self.subs = functools.partial(self.regexp.sub, r'\\\1')
        self.prepend = '"'
        self.append = '"'

    def repr(self, s):
        return self.prepend + self.subs(s) + self.append


def main():
    decoder = Decoder()
    with open('C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day8\\input.txt') as f:
        print(sum(len(decoder.repr(_[:-1])) - len(_[:-1]) for _ in f))


if __name__ == '__main__':
    main()
