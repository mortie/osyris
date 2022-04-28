#!/usr/bin/env python3

import re
import sys
import os

doc_rx = re.compile(r"^/\*\n(@.+?)^\*/", re.MULTILINE | re.DOTALL)
name_rx = re.compile(r"^@\((.*?)\s")
examples_rx = re.compile(r"^Examples:\n(.*)", re.MULTILINE | re.DOTALL)

if len(sys.argv) != 3:
    print("Usage:", sys.argv[0], "<infile> <outfile>")
    exit(1)

inpath = sys.argv[1]
outpath = sys.argv[2]

outfile = open(outpath + "~", "w")
outfile.write("; This test is auto-generated from doctest.py\n")
outfile.write("; based on: " + inpath + "\n")

class Reader:
    def __init__(self, content):
        self.content = content
        self.idx = 0

    def eof(self):
        return self.idx >= len(self.content)

    def peek(self, count=1):
        if self.idx + count >= len(self.content) + 1:
            return None
        return self.content[self.idx:self.idx + count]

    def consume(self, count=1):
        x = self.peek(count)
        while count > 0:
            if not self.eof():
                self.idx += 1
            count -= 1
        return x

    def skip_space(self):
        while not self.eof() and self.peek().isspace():
            self.consume()

def read_expr(r):
    r.skip_space()
    exprstart = r.idx
    ch = r.peek()
    if ch == '(' or ch == '[':
        depth = 1
        exprstart = r.idx
        r.consume()
        while not r.eof() and depth > 0:
            ch = r.consume()
            if ch == '(' or ch == '[':
                depth += 1
            elif ch == ')' or ch == ']':
                depth -= 1
        return r.content[exprstart:r.idx]
    elif ch == '"':
        r.consume()
        while not r.eof():
            ch = r.consume()
            if ch == "\"":
                break
            elif ch == "\\":
                r.consume() # Ignore this character
        return r.content[exprstart:r.idx]
    else:
        while not r.eof():
            ch = r.peek()
            if ch == "\"":
                read_expr(r)
            elif ch == ".":
                r.consume()
                read_expr(r)
            elif ch.isspace():
                break
            else:
                r.consume()
        return r.content[exprstart:r.idx]

def writexpr(f, expr):
    for l in expr.split("\n"):
        f.write("\t" + l.replace("    ", "\t") + "\n")

def gen_test(name, content):
    r = Reader(content)

    outfile.write("\n(test-case '" + name + " {\n")

    while not r.eof():
        r.skip_space()
        if r.peek() == ';':
            while r.peek() != '\n':
                r.consume()
            continue

        expr = read_expr(r).strip()
        if expr == "":
            continue
        r.skip_space()
        if r.peek(2) == '->':
            r.consume(2)
            compare = read_expr(r).strip()
            writexpr(outfile ,"(asserteq " + expr + " " + compare + ")")
        else:
            writexpr(outfile, expr)

    outfile.write("})\n")

content = open(sys.argv[1]).read()
for match in doc_rx.finditer(content):
    doc = match.group(1)
    name = name_rx.search(doc)
    examples = examples_rx.search(doc)
    if examples is not None:
        gen_test(name.group(1), examples.group(1))

os.rename(outpath + "~", outpath)
