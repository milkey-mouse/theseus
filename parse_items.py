#!/usr/bin/env python3
from html.parser import HTMLParser
from io import StringIO
import re
import sys


class ItemParser(HTMLParser):
    MISSING_SEMICOLONS = re.compile(r"((^|\n)(?:[^/]|/[^/])*[^};\s])(\s*(?://.*)?)$")

    def __init__(self, callback):
        super().__init__()

        self.callback = callback
        self.item = None
        self.in_code_attribute = False
        self.in_summary = False

    @staticmethod
    def classes(attrs):
        try:
            return next(v for k, v in reversed(attrs) if k == "class").split()
        except StopIteration:
            return ()

    def handle_starttag(self, tag, attrs):
        if self.item is not None:
            if tag == "div":
                classes = self.classes(attrs)
                if "code-attribute" in classes:
                    self.in_code_attribute = True
            elif tag == "summary":
                self.in_summary = True
        elif tag == "pre":
            classes = self.classes(attrs)
            if "rust" in classes and "item-decl" in classes:
                self.item = StringIO()

    def handle_endtag(self, tag):
        if self.item is not None:
            if tag == "pre":
                item = self.item.getvalue()
                self.callback(self.MISSING_SEMICOLONS.sub(r"\1;\2", item))
                self.item = None
            elif self.in_code_attribute and tag == "div":
                self.in_code_attribute = False
                self.item.write("\n")
            elif self.in_summary and tag == "summary":
                self.in_summary = False

    def handle_data(self, data):
        if self.item is not None and not self.in_summary:
            self.item.write(data)

    @classmethod
    def parse_file(cls, callback, path):
        parser = cls(callback)
        with open(path, "r") as f:
            parser.feed(f.read())


def main():
    if len(sys.argv) < 2:
        print("Usage: ./parse_items.py <struct.A.html> <static.B.html> <fn.C.html> ...")
        sys.exit(1)

    last_item = None

    def callback(item):
        nonlocal last_item
        if last_item:
            print(last_item, end="\n\n")
        last_item = item

    for path in sys.argv[1:]:
        ItemParser.parse_file(callback, path)

    if last_item:
        print(last_item)


if __name__ == "__main__":
    main()
