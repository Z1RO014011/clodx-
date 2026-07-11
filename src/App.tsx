import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { formatPercent, formatResetTime } from './lib/format';

export type UsageWindow = { remainingPercent: number; resetsAt?: string; windowSeconds: number };
export type ProviderSnapshot = { provider: string; displayName: string; plan?: string; shortWindow?: UsageWindow; weeklyWindow?: UsageWindow; resetCredits?: number; resetCreditExpiresAt: string[]; updatedAt: string; status: 'ok' | 'signed_out' | 'unavailable'; message?: string | null };
type Props = { initialSnapshot?: ProviderSnapshot };

export function StatusDock({ snapshot, onOpen }: { snapshot?: ProviderSnapshot; onOpen: () => void }) {
  const short = snapshot?.status === 'ok' ? snapshot.shortWindow : undefined;
  const weekly = snapshot?.status === 'ok' ? snapshot.weeklyWindow : undefined;
  return <button className="status-dock" onClick={onOpen} aria-label="打开 Clodx 额度详情"><span className="dock-glyph">›_</span><span className="dock-values"><strong>5H {short ? formatPercent(short.remainingPercent) : '--'}</strong><strong>WK {weekly ? formatPercent(weekly.remainingPercent) : '--'}</strong></span></button>;
}

function Window({ label, code, value }: { label: string; code: string; value?: UsageWindow }) {
  return <section className="quota-window"><div className="window-heading"><span className="metric-code">{code}</span><strong>{value ? formatPercent(value.remainingPercent) : '—'}</strong></div><span className="metric-label">{label}</span><div className="bar" aria-hidden="true"><i style={{ width: `${value?.remainingPercent ?? 0}%` }} /></div><p>重置于 {formatResetTime(value?.resetsAt, 'zh-CN')}</p></section>;
}

export default function App({ initialSnapshot }: Props) {
  const [snapshot, setSnapshot] = useState<ProviderSnapshot | undefined>(initialSnapshot);
  const [loading, setLoading] = useState(!initialSnapshot);
  const refresh = async () => { setLoading(true); try { const values = await invoke<ProviderSnapshot[]>('refresh_snapshots'); const next = values[0]; setSnapshot(next); await invoke('set_tray_quota', { percent: next?.shortWindow?.remainingPercent }); } catch { setSnapshot({ provider: 'codex', displayName: 'CODEX', resetCreditExpiresAt: [], updatedAt: new Date().toISOString(), status: 'unavailable', message: 'Could not refresh quota.' }); } finally { setLoading(false); } };
  useEffect(() => { if (!initialSnapshot) void refresh(); }, []);
  if (window.location.hash === '#status') return <StatusDock snapshot={snapshot} onOpen={() => { void invoke('show_main_window'); }} />;
  if (loading && !snapshot) return <main className="widget" aria-busy="true"><p className="eyebrow">CLODX / CONNECTING</p><h1>正在读取额度…</h1></main>;
  if (!snapshot || snapshot.status !== 'ok') return <main className="widget"><p className="eyebrow">CLODX / STATUS</p><h1>{snapshot?.status === 'signed_out' ? '需要登录 Codex' : '额度暂不可用'}</h1><p className="message">{snapshot?.message ?? '正在等待额度数据。'}</p><button onClick={refresh} disabled={loading}>{loading ? '刷新中…' : '重新连接'}</button></main>;
  return <main className="widget"><header><div className="terminal-mark" aria-hidden="true">›_</div><div><p className="eyebrow">{snapshot.displayName} / USAGE</p><h1>{snapshot.plan ?? '额度监控'}</h1></div><button onClick={refresh} disabled={loading} aria-label="刷新额度">{loading ? '…' : '↻'}</button></header><section className="primary"><span className="eyebrow">PRIMARY WINDOW / 5H</span><strong>{snapshot.shortWindow ? formatPercent(snapshot.shortWindow.remainingPercent) : '—'}</strong><span className="metric-label">5 小时额度</span><p>下次重置 · {formatResetTime(snapshot.shortWindow?.resetsAt, 'zh-CN')}</p></section><div className="quota-grid"><Window code="WK" label="周额度" value={snapshot.weeklyWindow} /><section className="credits"><span className="metric-code">RC</span><strong>{snapshot.resetCredits === undefined ? '—' : String(snapshot.resetCredits).padStart(2, '0')}</strong><span className="metric-label">可用重置次数</span><p>{snapshot.resetCredits === undefined ? '服务未提供' : `可用重置次数：${snapshot.resetCredits}`}</p></section></div><footer><span>SYNCED · {formatResetTime(snapshot.updatedAt, 'zh-CN')}</span><span>LOCAL ONLY</span></footer></main>;
}
