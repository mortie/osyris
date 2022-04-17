#!/usr/bin/env python3

import re
import sys
import os

doc_rx = re.compile(r"^/\*\n(@.+?)^\*/", re.MULTILINE | re.DOTALL)
name_rx = re.compile(r"^@\((.*?)\s")
examples_rx = re.compile(r"^Examples:\n(.*)", re.MULTILINE | re.DOTALL)

if len(sys.argv) != 3:
    print("Usage:", sys.argv[0], "<infile> <outfile>")

inpath = sys.argv[1]
outpath = sys.argv[2]

outfile = open(outpath + "~", "w")
outfile.write("; This test is auto-generated from doctest.py\n")
outfile.write("; based on: " + inpath + "\n")

def gen_test(name, content):
    outfile.write("\n(test-case '" + name + " {\n")

    idx = 0
    depth = 0
    exprstart = None
    while idx < len(content):
        if content[idx] == '(':
            if depth == 0:
                exprstart = idx
            depth += 1
            idx += 1
        elif content[idx] == ')':
            depth -= 1

            if depth == 0:
                idx += 1
                expr = content[exprstart:idx]

                # Skip whitespace
                while idx < len(content) and content[idx].isspace():
                    idx += 1

                # If the stuff right after an expression is "->",
                # this is something we want to assert
                if content[idx:idx+2] == "->":
                    # Skip '->' followed by spaces
                    idx += 2
                    while idx < len(content) and content[idx] == ' ':
                        idx += 1

                    # Everything after '->' until the newline is an expression
                    start = idx
                    while idx < len(content) and content[idx] != '\n':
                        idx += 1
                    compare = content[start:idx]

                    expr = "(asserteq " + expr + " " + compare + ")"
                    idx += 1

                for l in expr.split("\n"):
                    outfile.write("\t" + l.replace("    ", "\t") + "\n")

                if content[idx:idx+1] == "\n":
                    outfile.write("\n")
            else:
                idx += 1
        else:
            idx += 1

    outfile.write("})\n")

content = open(sys.argv[1]).read()
for match in doc_rx.finditer(content):
    doc = match.group(1)
    name = name_rx.search(doc)
    examples = examples_rx.search(doc)
    if examples is not None:
        gen_test(name.group(1), examples.group(1))

os.rename(outpath + "~", outpath)
