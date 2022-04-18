#!/usr/bin/env python3

import re
import sys
import os

doc_rx = re.compile(r"^/\*\n(@.+?)^\*/", re.MULTILINE | re.DOTALL)
signature_rx = re.compile(r"^@(.*?)\n")
desc_and_examples_rx = re.compile(r"^.*?\n(.*)^Examples:\n(.*)", re.MULTILINE | re.DOTALL)
desc_rx = re.compile(r"^.*?\n(.*)", re.MULTILINE | re.DOTALL)
name_rx = re.compile(r"^\((.*?) ")

def slugify_simple(title):
    slug = ""
    for ch in title.strip():
        if ch.isalnum() or ch == "-":
            slug += ch
        elif ch.isspace():
            slug += "-"
    return slug

slugs = {}
def slugify(title):
    slug = slugify_simple(title)
    if slug in slugs:
        num = slugs[slug]
        slugs[slug] += 1
        slug += "-" + str(num)
    else:
        slugs[slug] = 1
    return slug

def mdescape(s):
    return s.replace("*", "\\*").replace("_", "\\_")

if len(sys.argv) != 4:
    print("Usage:", sys.argv[0], "<infile> <outfile> <name>")
    exit(1)

inpath = sys.argv[1]
outpath = sys.argv[2]
name = sys.argv[3]

documentation = ""
table_of_contents = ""

content = open(sys.argv[1]).read()
for match in doc_rx.finditer(content):
    doc = match.group(1)
    signature = signature_rx.search(doc).group(1).strip()
    func_name = name_rx.search(signature).group(1)

    desc_and_examples = desc_and_examples_rx.search(doc)
    if desc_and_examples is not None:
        desc, examples = desc_and_examples.groups()
    else:
        desc = desc_rx.search(doc).group(1)
        examples = None

    title = ": " + mdescape(func_name)
    table_of_contents += "* [" + title + "](#" + slugify(title) + ")\n"

    documentation += "---\n\n### " + title + "\n\n"
    documentation += "    " + signature + "\n\n"
    documentation += desc.strip() + "\n\n"
    if examples is not None:
        documentation += "Examples:\n\n"
        for line in examples.strip().split("\n"):
            if line == "":
                documentation += "\n"
            else:
                documentation += "    " + line + "\n"
        documentation += "\n"

outfile = open(outpath, "w")
outfile.write("# " + name + "\n\n")
outfile.write(table_of_contents.strip() + "\n\n")
outfile.write(documentation.strip() + "\n")
