export const LIST_VIEW_MODES = ['table', 'grid', 'list'] as const

export type ListViewMode = (typeof LIST_VIEW_MODES)[number]
