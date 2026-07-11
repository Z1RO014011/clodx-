# Clodx Status Bar Implementation Plan

**Goal:** Add a native status-bar/tray percentage entry that opens the Clodx widget.

1. Add a failing Rust test for formatting a valid 5-hour quota as `C 73%` and unavailable data as `C --`.
2. Implement the pure formatter and a `set_tray_quota` Tauri command which updates the tray title/tooltip.
3. Create the tray menu and left-click Show/Hide behavior in the native app setup.
4. Call `set_tray_quota` after each successful frontend snapshot refresh.
5. Run frontend and Rust tests plus `cargo check`.
