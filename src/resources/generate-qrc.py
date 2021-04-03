#!/usr/bin/env python3
# Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

# Scan speicified folder and generate qrc resource file.

import argparse
import os
import sys

def generate_header(fd, prefix):
    fd.write("<!DOCTYPE RCC>\n")
    fd.write('<RCC version="1.0">\n')
    fd.write('  <qresource prefix="/%s">\n' % prefix)

def generate_tail(fd):
    fd.write("  </qresource>\n")
    fd.write("</RCC>\n")

def generate_entry(fd, folder, entry):
    fd.write("    <file>%s</file>\n" % entry[len(folder) + 1:])

def generate_empty_entry(fd):
    fd.write("\n")

def generate_qrc(folder, prefix, output):
    folder = os.path.abspath(folder)
    out_fd = sys.stdout
    if output:
        out_fd = open(output, "w")

    if not prefix:
        prefix = os.path.basename(folder)
    generate_header(out_fd, prefix)
    for root, dirs, files in os.walk(folder, topdown=True):
        for filename in sorted(files):
            entry = os.path.join(root, filename)
            generate_entry(out_fd, folder, entry)
        generate_empty_entry(out_fd)
    generate_tail(out_fd)

    if output:
        out_fd.close()

def main():
    parser = argparse.ArgumentParser(description="Generate qrc file")
    parser.add_argument("-p", "--prefix", metavar="PREFIX",
                        help="Set resource prefix")
    parser.add_argument("-o", "--output", metavar="OUTPUT",
                        help="Output to file")
    parser.add_argument("folder", type=str,
                        help="Source folder")
    args = parser.parse_args()
    generate_qrc(args.folder, args.prefix, args.output)


if __name__ == "__main__":
    main()
