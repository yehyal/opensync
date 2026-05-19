# OpenSync

OpenSync is a workspace for a cross-device sync product that currently focuses on clipboard syncing, with storage support already wired into the desktop apps.

## Included apps

- `apps/desktop` - native Rust desktop app built with `tao` and tray support. (Old Version of App, Deprecated)
- `apps/desktop-tauri` - Tauri desktop app with a React frontend.
- `apps/api` - backend app directory, currently present but still mostly starter scaffolding.
- `apps/web` - web app directory, currently present but still mostly starter scaffolding.

## Crates currently used

The workspace has multiple crates under `crates/`, but the ones in active use right now are:

- `clipboard`
- `storage`

## Workspace

- Root Cargo workspace members include the Rust desktop app, the Tauri app, and crates under `crates/*`.
- Frontend tooling is managed with `pnpm` and `turbo`.

## Running

```bash
pnpm dev
```

For the Tauri desktop app:

```bash
cd apps/desktop-tauri
pnpm tauri dev
```
