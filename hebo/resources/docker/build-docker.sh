#!/bin/bash
# Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

set -xe

readonly GID=$(id -g)
sudo docker run --user ${UID}:${GID} --rm --volume ${PWD}/../../../:/hebo \
  rust:latest /bin/bash -c 'cd /hebo/hebo; cargo build --release --bin hebo'
#cd ../../.. && cross build --release --bin hebo --target x86_64-unknown-linux-gnu

install -m644 ../pifu/hebo.toml hebo.toml
install -m755 ../../../target/release/hebo hebo
sudo docker build -t hebo:latest .
