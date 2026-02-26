export const LIST_VIEW_MODES = ['table', 'board', 'list'] as const

export type ListViewMode = (typeof LIST_VIEW_MODES)[number]
