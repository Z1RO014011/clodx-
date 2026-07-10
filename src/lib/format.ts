export function formatPercent(value: number): string {
  const normalized = Math.min(100, Math.max(0, value));
  return `${Number.isInteger(normalized) ? normalized : normalized.toFixed(1)}%`;
}

export function formatResetTime(value: string | undefined, locale: 'en' | 'zh-CN'): string {
  if (!value) return '—';

  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return '—';

  return new Intl.DateTimeFormat(locale, {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  }).format(date);
}
