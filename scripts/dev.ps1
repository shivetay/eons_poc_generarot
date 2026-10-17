# Uruchamia aplikację bez trunk serve / 127.0.0.1:1420.
# Frontend jest budowany do dist/ i serwowany przez Tauri.
Set-Location (Split-Path -Parent $PSScriptRoot)
cargo tauri dev
