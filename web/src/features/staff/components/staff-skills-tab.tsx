import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import type { StaffSkill } from '@/lib/api'
import {
  getStaffSkillsQueryOptions,
  useCreateStaffSkill,
  useDeleteStaffSkill,
} from '@/features/staff/api'
import { authClient } from '@/lib/clients'
import { Button } from '@/components/ui/button'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { HStack, Stack, Text } from '@/components/primitives'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Badge } from '@/components/ui/badge'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'

interface StaffSkillsTabProps {
  staffId?: string
}

export function StaffSkillsTab({ staffId }: StaffSkillsTabProps) {
  const [isCreateOpen, setIsCreateOpen] = React.useState(false)
  const [isDeleteOpen, setIsDeleteOpen] = React.useState(false)
  const [skillToDelete, setSkillToDelete] = React.useState<string | null>(null)

  const { data, isLoading } = useQuery({
    ...getStaffSkillsQueryOptions({
      client: authClient,
      query: staffId ? { filters: { staff_id: staffId } } : {},
    }),
  })

  const createSkill = useCreateStaffSkill()
  const deleteSkill = useDeleteStaffSkill()

  const [formData, setFormData] = React.useState({
    staff_id: staffId || '',
    skill_name: '',
    proficiency_level: 'Intermediate',
    notes: '',
  })

  const handleSubmit = () => {
    createSkill.mutate(formData, {
      onSuccess: () => {
        setIsCreateOpen(false)
        setFormData({
          staff_id: staffId || '',
          skill_name: '',
          proficiency_level: 'Intermediate',
          notes: '',
        })
      },
    })
  }

  const handleDelete = () => {
    if (skillToDelete) {
      deleteSkill.mutate(
        { id: skillToDelete },
        {
          onSuccess: () => {
            setIsDeleteOpen(false)
            setSkillToDelete(null)
          },
        },
      )
    }
  }

  return (
    <Card>
      <CardHeader>
        <HStack justify="between" align="center">
          <Stack gap={1}>
            <CardTitle>Skills</CardTitle>
            <CardDescription>
              Manage professional skills and competencies
            </CardDescription>
          </Stack>
          <Button size="sm" onClick={() => setIsCreateOpen(true)}>
            <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
            Add Skill
          </Button>
        </HStack>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[400px]">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Skill</TableHead>
                <TableHead>Proficiency</TableHead>
                <TableHead>Notes</TableHead>
                <TableHead className="w-[100px]">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {isLoading ? (
                <TableRow>
                  <TableCell colSpan={4} className="text-center py-8">
                    <Text muted>Loading...</Text>
                  </TableCell>
                </TableRow>
              ) : !data?.data || data.data.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={4} className="text-center py-8">
                    <Text muted>No skills found</Text>
                  </TableCell>
                </TableRow>
              ) : (
                data.data.map((skill: StaffSkill) => (
                  <TableRow key={skill.id}>
                    <TableCell className="font-medium">{skill.skill_name}</TableCell>
                    <TableCell>
                      <Badge
                        variant="outline"
                        className={
                          skill.proficiency_level === 'Expert'
                            ? 'bg-purple-500/10 text-purple-500'
                            : skill.proficiency_level === 'Advanced'
                            ? 'bg-blue-500/10 text-blue-500'
                            : skill.proficiency_level === 'Intermediate'
                            ? 'bg-green-500/10 text-green-500'
                            : 'bg-gray-500/10 text-gray-500'
                        }
                      >
                        {skill.proficiency_level}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      <Text size="sm" muted className="max-w-[200px] truncate">
                        {skill.notes || '—'}
                      </Text>
                    </TableCell>
                    <TableCell>
                      <Button
                        variant="ghost"
                        size="icon"
                        onClick={() => {
                          setSkillToDelete(skill.id)
                          setIsDeleteOpen(true)
                        }}
                      >
                        <HugeiconsIcon icon={Delete02Icon} className="size-4" />
                      </Button>
                    </TableCell>
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </ScrollArea>
      </CardContent>

      {/* Create Dialog */}
      <Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Add Skill</DialogTitle>
            <DialogDescription>
              Add a new professional skill
            </DialogDescription>
          </DialogHeader>
          <Stack gap={4}>
            <div className="grid gap-2">
              <Label htmlFor="skill">Skill Name</Label>
              <Input
                id="skill"
                value={formData.skill_name}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, skill_name: e.target.value }))
                }
                placeholder="e.g., Classroom Management, Python Programming"
                required
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="proficiency">Proficiency Level</Label>
              <Select
                value={formData.proficiency_level}
                onValueChange={(value) => {
                  if (value) setFormData((prev) => ({ ...prev, proficiency_level: value }))
                }}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="Beginner">Beginner</SelectItem>
                  <SelectItem value="Intermediate">Intermediate</SelectItem>
                  <SelectItem value="Advanced">Advanced</SelectItem>
                  <SelectItem value="Expert">Expert</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div className="grid gap-2">
              <Label htmlFor="notes">Notes (optional)</Label>
              <Input
                id="notes"
                value={formData.notes}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, notes: e.target.value }))
                }
                placeholder="Additional details about this skill"
              />
            </div>
          </Stack>
          <DialogFooter>
            <Button variant="outline" onClick={() => setIsCreateOpen(false)}>
              Cancel
            </Button>
            <Button onClick={handleSubmit} disabled={createSkill.isPending}>
              {createSkill.isPending ? 'Saving...' : 'Save'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation */}
      <AlertDialog open={isDeleteOpen} onOpenChange={setIsDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Skill</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this skill? This action cannot
              be undone.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={handleDelete}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </Card>
  )
}
