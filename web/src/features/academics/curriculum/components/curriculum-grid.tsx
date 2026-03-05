import { CurriculumCard } from './curriculum-card'
import type { CurriculumStandardResponse } from '@/lib/api/types.gen'
import { Grid } from '@/components/primitives'

interface CurriculumGridProps {
  standards: Array<
    CurriculumStandardResponse & {
      subject_name?: string
      grade_level_name?: string
    }
  >
  onEdit: (standard: CurriculumStandardResponse) => void
  onDelete: (id: string) => void
}

export function CurriculumGrid({
  standards,
  onEdit,
  onDelete,
}: CurriculumGridProps) {
  return (
    <Grid gap={4} className="grid-cols-1 md:grid-cols-2 xl:grid-cols-3">
      {standards.map((standard) => (
        <CurriculumCard
          key={standard.id}
          standard={standard}
          onEdit={onEdit}
          onDelete={onDelete}
        />
      ))}
    </Grid>
  )
}
