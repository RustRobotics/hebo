// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "mqtt/connection_state.h"

namespace hebo {
namespace {

const char* dumpState(ConnectionState state) {
  switch (state) {
    case ConnectionState::kConnectFailed: {
      return "connectFailed";
    }
    case ConnectionState::kConnected: {
      return "connected";
    }
    case ConnectionState::kConnecting: {
      return "connecting";
    }
    case ConnectionState::kDisconnected: {
      return "disconnected";
    }
    case ConnectionState::kDisconnecting: {
      return "disconnecting";
    }
    default: {
      return "";
    }
  }
}

}  // namespace

QDebug operator<<(QDebug stream, ConnectionState state) {
  stream << dumpState(state);
  return stream;
}

}  // namespace hebo
