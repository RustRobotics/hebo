#!/bin/bash
# Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

set -xe

GID=$(id -g)
sudo docker run --user ${UID}:${GID} --rm --volume ${PWD}/../../:/hebo \
  rust:latest /bin/bash -c 'cd /hebo; cargo build'

