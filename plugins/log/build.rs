// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

const COMMANDS: &[&str] = &["log"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).ios_path("ios").build();
}
