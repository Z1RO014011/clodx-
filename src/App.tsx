import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { formatPercent, formatResetTime } from './lib/format';

export type UsageWindow = { remainingPercent: number; resetsAt?: string; windowSeconds: number };
export type ProviderSnapshot = { provider: string; displayName: string; plan?: string; shortWindow?: UsageWindow; weeklyWindow?: UsageWindow; resetCredits?: number; resetCreditExpiresAt: string[]; updatedAt: string; status: 'ok' | 'signed_out' | 'unavailable'; message?: string | null };
type Props = { initialSnapshot?: ProviderSnapshot };

function Window({ label, value }: { label: string; value?: UsageWindow }) {
  return <section className="quota-window"><div className="window-heading"><span>{label}</span><strong>{value ? formatPercent(value.remainingPercent) : '—'}</strong></div><div className="bar" aria-hidden="true"><i style={{ width: `${value?.remainingPercent ?? 0}%` }} /></div><p>Resets {formatResetTime(value?.resetsAt, 'en')}</p></section>;
}

export default function App({ initialSnapshot }: Props) {
  const [snapshot, setSnapshot] = useState<ProviderSnapshot | undefined>(initialSnapshot);
  const [loading, setLoading] = useState(!initialSnapshot);
  const refresh = async () => { setLoading(true); try { const values = await invoke<ProviderSnapshot[]>('refresh_snapshots'); setSnapshot(values[0]); } catch { setSnapshot({ provider: 'codex', displayName: 'CODEX', resetCreditExpiresAt: [], updatedAt: new Date().toISOString(), status: 'unavailable', message: 'Could not refresh quota.' }); } finally { setLoading(false); } };
  useEffect(() => { if (!initialSnapshot) void refresh(); }, []);
  if (loading && !snapshot) return <main className="widget" aria-busy="true"><p className="eyebrow">CLODX</p><h1>Checking quota…</h1></main>;
  if (!snapshot || snapshot.status !== 'ok') return <main className="widget"><p className="eyebrow">CLODX</p><h1>{snapshot?.status === 'signed_out' ? 'Sign in required' : 'Quota unavailable'}</h1><p className="message">{snapshot?.message ?? 'Waiting for quota data.'}</p><button onClick={refresh} disabled={loading}>{loading ? 'Refreshing…' : 'Retry'}</button></main>;
  return <main className="widget"><header><div><p className="eyebrow">{snapshot.displayName}</p><h1>{snapshot.plan ?? 'Codex quota'}</h1></div><button onClick={refresh} disabled={loading} aria-label="Refresh quota">{loading ? '…' : 'Refresh'}</button></header><Window label="5-hour quota" value={snapshot.shortWindow} /><Window label="Weekly quota" value={snapshot.weeklyWindow} /><section className="credits"><span>Reset credits</span><strong>{snapshot.resetCredits === undefined ? 'Not available' : `${snapshot.resetCredits} reset credits`}</strong></section><footer>Updated {formatResetTime(snapshot.updatedAt, 'en')}</footer></main>;
}
