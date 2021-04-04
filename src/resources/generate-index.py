#!/usr/bin/env python3
# Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

import os
import sys

def parse_folder(target_dir):
    if target_dir.endswith("/"):
        target_dir = target_dir[:-1]
    files = sorted(os.listdir(target_dir))
    parts = target_dir.removeprefix(".").split("/")
    parts[0] = parts[0].removesuffix("s")

    prefix = "".join(part.capitalize() for part in parts)
    for filename in files:
        name_parts = os.path.splitext(filename)[0].split("-")
        key = "".join(part.capitalize() for part in name_parts)
        value = ":/{}/{}".format(target_dir, filename)
        item = 'constexpr const char* k{0}{1} = "{2}";'.format(prefix, key, value)
        print(item)

def main():
    if len(sys.argv) != 2:
        print("Usage: %s resource-folder" % sys.argv[0])
        return
    target_dir = sys.argv[1]
    parse_folder(target_dir)


if __name__ == "__main__":
    main()
