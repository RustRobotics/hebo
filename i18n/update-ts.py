#!/usr/bin/env python3
# Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.


import subprocess

def generate_qm(lang):
    ts_file = "i18n/hebo-{lang}.ts".format(lang=lang)
    cmd = [
            "lupdate", "-recursive", "-I.",
            "src/app",
            "src/controllers",
            "src/ui",
            "-ts", ts_file,
        ]
    subprocess.call(cmd)
    return

    # Fix namespace mssing in ts files.
    lines = []
    with open(ts_file) as fh:
        for line in fh:
            if "<name>" in line and "QObject" not in line:
                line = line.replace("<name>", "<name>hebo::")
            lines.append(line)
    with open(ts_file, "w") as fh:
        for line in lines:
            fh.write(line)

def main():
    for lang in ( "zh_CN", "en_US"):
        generate_qm(lang)

if __name__ == "__main__":
    main()
