// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { invoke } from "@tauri-apps/api/core";
import { WindowLabel, getCurrent } from "@tauri-apps/api/window";

export enum StateFlags {
  SIZE = 1 << 0,
  POSITION = 1 << 1,
  MAXIMIZED = 1 << 2,
  VISIBLE = 1 << 3,
  DECORATIONS = 1 << 4,
  FULLSCREEN = 1 << 5,
  ALL = SIZE | POSITION | MAXIMIZED | VISIBLE | DECORATIONS | FULLSCREEN,
}

/**
 *  Save the state of all open windows to disk.
 */
async function saveWindowState(flags: StateFlags): Promise<void> {
  return invoke("plugin:window-state|save_window_state", { flags });
}

/**
 *  Restore the state for the specified window from disk.
 */
async function restoreState(
  label: WindowLabel,
  flags: StateFlags,
): Promise<void> {
  return invoke("plugin:window-state|restore_state", { label, flags });
}

/**
 *  Restore the state for the current window from disk.
 */
async function restoreStateCurrent(flags: StateFlags): Promise<void> {
  return restoreState(getCurrent().label, flags);
}

export { restoreState, restoreStateCurrent, saveWindowState };
