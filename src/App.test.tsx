import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import App, { StatusDock, type ProviderSnapshot } from './App';

const snapshot: ProviderSnapshot = {
  provider: 'codex', displayName: 'CODEX', plan: 'PLUS',
  shortWindow: { remainingPercent: 73.4, resetsAt: '2026-07-11T12:00:00Z', windowSeconds: 18000 },
  weeklyWindow: { remainingPercent: 48, resetsAt: '2026-07-14T12:00:00Z', windowSeconds: 604800 },
  resetCredits: 2, resetCreditExpiresAt: [], updatedAt: '2026-07-11T10:00:00Z', status: 'ok', message: null,
};

describe('Clodx widget', () => {
  it('shows compact Chinese quota windows and available reset credits', () => {
    render(<App initialSnapshot={snapshot} />);
    expect(screen.getByText('5 小时额度')).toBeInTheDocument();
    expect(screen.getByText('73.4%')).toBeInTheDocument();
    expect(screen.getByText('周额度')).toBeInTheDocument();
    expect(screen.getByText('可用重置次数：2')).toBeInTheDocument();
  });

  it('renders the compact two-line status dock', () => {
    render(<StatusDock snapshot={snapshot} onOpen={() => undefined} />);
    expect(screen.getByText('5H 73.4%')).toBeInTheDocument();
    expect(screen.getByText('WK 48%')).toBeInTheDocument();
  });
});
