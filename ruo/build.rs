// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Affero General Public License that can be found
// in the LICENSE file.

use protobuf_codegen_pure::Customize;

fn main() {
    protobuf_codegen_pure::Codegen::new()
        .customize(Customize {
            ..Default::default()
        })
        .out_dir("examples/protobuf/protos")
        .input("examples/protobuf/protos/geometry.proto")
        .include("examples/protobuf/protos")
        .run()
        .expect("protoc");
}
