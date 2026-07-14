import { describe, expect, it } from 'vitest';
import { formatPercent, formatResetTime } from './format';

describe('quota formatting', () => {
  it('renders a remaining percentage with one decimal place only when needed', () => {
    expect(formatPercent(73.4)).toBe('73.4%');
    expect(formatPercent(60)).toBe('60%');
  });

  it('renders an unavailable reset as a dash', () => {
    expect(formatResetTime(undefined, 'en')).toBe('—');
  });
});
