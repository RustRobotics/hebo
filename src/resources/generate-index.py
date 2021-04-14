#!/usr/bin/env python3
# Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import os
import sys

def generate_folder_comment(fd, folder):
    fd.write("// {}/\n".format(folder));

def generate_end_of_folder(fd):
    fd.write("\n");

def generate_entry(fd, folder, filename):
    parts = folder.removeprefix(".").split("/")
    parts[0] = parts[0].removesuffix("s")
    prefix = "".join(part.capitalize() for part in parts)
    name_parts = os.path.splitext(filename)[0].split("-")
    key = "".join(part.capitalize() for part in name_parts)
    value = ":/{}/{}".format(folder, filename)
    item = 'constexpr const char* k{0}{1} = "{2}";\n'.format(prefix, key, value)
    fd.write(item)

def walkdir(folder):
    abs_folder = os.path.abspath(folder)
    if folder.endswith("/"):
        folder = folder[:-1]
    out_fd = sys.stdout

    for root, dirs, files in os.walk(abs_folder, topdown=True):
        local_folder = root[len(abs_folder) - len(folder):]
        generate_folder_comment(out_fd, local_folder)
        for filename in sorted(files):
            generate_entry(out_fd, local_folder, filename)
        generate_end_of_folder(out_fd)

def main():
    if len(sys.argv) != 2:
        print("Usage: %s resource-folder" % sys.argv[0])
        return
    target_dir = sys.argv[1]
    walkdir(target_dir)


if __name__ == "__main__":
    main()
