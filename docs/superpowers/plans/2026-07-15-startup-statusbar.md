# Clodx 启动与状态栏实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 让 Clodx 静默驻留菜单栏、自动完成开发前端启动，并在菜单栏正确显示可用的 5H 或周额度。

**Architecture:** Tauri 配置负责开发服务器生命周期与默认隐藏的窗口；Rust 负责将两个可选额度窗口映射为简短菜单栏标题。前端刷新时将两个窗口一起交给 Rust，以便周额度成为唯一可用窗口时仍能更新标题。

**Tech Stack:** Tauri 2、Rust、Vite、React、Vitest、Cargo test。

---

## 文件结构

- 修改：`src-tauri/tauri.conf.json` — 启动 Vite 的 `beforeDevCommand`、统一本机开发 URL，保留隐藏主窗口。
- 修改：`src-tauri/src/lib.rs` — 从 5H、WK 窗口选择菜单栏标题并在刷新时更新。
- 修改：`src-tauri/tests/tray_title.rs` — 为额度标题优先级编写回归测试。
- 修改：`src/App.tsx` — 刷新时同时传递 5H 与周窗口百分比。
- 修改：`src/App.test.tsx` — 验证无 5H 时状态栏组件只显示 WK。

### Task 1: 菜单栏标题优先级

**Files:**
- Modify: `src-tauri/tests/tray_title.rs`
- Modify: `src-tauri/src/lib.rs:8-11,25-31`

- [ ] **Step 1: 写出失败测试**

把 `src-tauri/tests/tray_title.rs` 改为：

```rust
use clodx_lib::format_tray_title;

#[test]
fn prefers_five_hour_quota_and_falls_back_to_weekly_quota() {
    assert_eq!(format_tray_title(Some(73.4), Some(97.0)), "5H 73%");
    assert_eq!(format_tray_title(None, Some(97.0)), "WK 97%");
    assert_eq!(format_tray_title(None, None), "C --");
}
```

- [ ] **Step 2: 验证测试失败**

Run: `cargo test --manifest-path src-tauri/Cargo.toml prefers_five_hour_quota_and_falls_back_to_weekly_quota`

Expected: FAIL，原因是现有 `format_tray_title` 仅接受一个参数。

- [ ] **Step 3: 实现最小标题选择逻辑**

将 `src-tauri/src/lib.rs` 中的函数替换为：

```rust
pub fn format_tray_title(short_percent: Option<f64>, weekly_percent: Option<f64>) -> String {
    if let Some(value) = short_percent {
        return format!("5H {:.0}%", value.clamp(0.0, 100.0));
    }
    if let Some(value) = weekly_percent {
        return format!("WK {:.0}%", value.clamp(0.0, 100.0));
    }
    "C --".into()
}
```

把 `set_tray_quota` 改为接收 `short_percent: Option<f64>` 与 `weekly_percent: Option<f64>`，并用两者调用 `format_tray_title`。其余的菜单栏标题与 tooltip 设置逻辑不变。

- [ ] **Step 4: 验证测试通过**

Run: `cargo test --manifest-path src-tauri/Cargo.toml prefers_five_hour_quota_and_falls_back_to_weekly_quota`

Expected: PASS。

- [ ] **Step 5: 提交**

```bash
git add src-tauri/src/lib.rs src-tauri/tests/tray_title.rs
git commit -m "feat: show weekly quota in status bar"
```

### Task 2: 前端刷新传递两种额度

**Files:**
- Modify: `src/App.test.tsx`
- Modify: `src/App.tsx:20-22`

- [ ] **Step 1: 写出失败测试**

在 `src/App.test.tsx` 的无 5H 测试中，加入：

```tsx
const dock = render(<StatusDock snapshot={{ ...snapshot, shortWindow: undefined }} onOpen={() => undefined} />);
expect(within(dock.container).getByText('WK 48%')).toBeInTheDocument();
expect(within(dock.container).queryByText(/5H/)).not.toBeInTheDocument();
```

- [ ] **Step 2: 验证测试失败**

Run: `npm test -- src/App.test.tsx`

Expected: FAIL，因为新增测试尚未导入 `within` 或状态栏显示逻辑未验证到容器范围。

- [ ] **Step 3: 更新刷新调用**

在 `src/App.tsx` 的 `refresh` 内，把：

```tsx
await invoke('set_tray_quota', { percent: next?.shortWindow?.remainingPercent });
```

替换为：

```tsx
await invoke('set_tray_quota', {
  shortPercent: next?.shortWindow?.remainingPercent,
  weeklyPercent: next?.weeklyWindow?.remainingPercent,
});
```

若测试文件尚未导入 `within`，将首行改为：

```tsx
import { render, screen, within } from '@testing-library/react';
```

- [ ] **Step 4: 验证前端测试通过**

Run: `npm test -- src/App.test.tsx`

Expected: PASS。

- [ ] **Step 5: 提交**

```bash
git add src/App.tsx src/App.test.tsx
git commit -m "feat: pass weekly quota to status bar"
```

### Task 3: 自动化开发启动

**Files:**
- Modify: `src-tauri/tauri.conf.json`

- [ ] **Step 1: 写出配置断言脚本**

Run:

```bash
node -e 'const c=require("./src-tauri/tauri.conf.json"); if(c.build.beforeDevCommand!=="npm run dev -- --host 127.0.0.1"||c.build.devUrl!=="http://127.0.0.1:5173") process.exit(1)'
```

Expected: FAIL，因为当前配置没有 `beforeDevCommand`，且使用 `localhost`。

- [ ] **Step 2: 更新 Tauri 构建配置**

将 `build` 配置设置为：

```json
"build": {
  "beforeDevCommand": "npm run dev -- --host 127.0.0.1",
  "frontendDist": "../dist",
  "devUrl": "http://127.0.0.1:5173"
}
```

不要修改主窗口的 `visible: false`，以保持正式版静默驻留。

- [ ] **Step 3: 验证配置断言通过**

Run:

```bash
node -e 'const c=require("./src-tauri/tauri.conf.json"); if(c.build.beforeDevCommand!=="npm run dev -- --host 127.0.0.1"||c.build.devUrl!=="http://127.0.0.1:5173") process.exit(1)'
```

Expected: exit code 0。

- [ ] **Step 4: 手动开发启动验证**

Run: `npm run tauri dev`

Expected: Vite 自动启动，随后出现 `Running target/debug/clodx`；无需另开 `npm run dev`。

- [ ] **Step 5: 提交**

```bash
git add src-tauri/tauri.conf.json
git commit -m "fix: start Vite automatically for Tauri development"
```

### Task 4: 全量回归

**Files:**
- Verify only: `src-tauri/src/codex.rs`, `src-tauri/src/lib.rs`, `src/App.tsx`

- [ ] **Step 1: 运行 Rust 全量测试**

Run: `cargo test --manifest-path src-tauri/Cargo.toml`

Expected: 所有 Rust 测试 PASS，包括周额度不被分类为 5H。

- [ ] **Step 2: 运行前端全量测试与构建**

Run: `npm test && npm run build && git diff --check`

Expected: 全部 PASS，且没有 whitespace 错误。

- [ ] **Step 3: 提交先前修正的周额度解析**

```bash
git add src-tauri/src/codex.rs
git commit -m "fix: classify weekly primary usage correctly"
```

