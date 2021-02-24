// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/conn_state.h"

namespace hebo {
namespace {

const char* dumpState(ConnectState state) {
  switch (state) {
    case ConnectState::kConnectFailed: {
      return "connectFailed";
    }
    case ConnectState::kConnected: {
      return "connected";
    }
    case ConnectState::kConnecting: {
      return "connecting";
    }
    case ConnectState::kDisconnected: {
      return "disconnected";
    }
    case ConnectState::kDisconnecting: {
      return "disconnecting";
    }
    default: {
      return "";
    }
  }
}

}  // namespace

QDebug operator<<(QDebug stream, ConnectState state) {
  stream << dumpState(state);
  return stream;
}

}  // namespace hebo