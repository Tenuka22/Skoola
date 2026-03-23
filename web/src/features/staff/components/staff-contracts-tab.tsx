// Simplified staff contracts tab - matches actual API schema
import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, PencilEdit01Icon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import { format } from 'date-fns'
import type { StaffContractResponse } from '@/lib/api'
import {
  getStaffContractsQueryOptions,
  useCreateStaffContract,
  useDeleteStaffContract,
  useUpdateStaffContract,
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

interface StaffContractsTabProps {
  staffId?: string
}

export function StaffContractsTab({ staffId }: StaffContractsTabProps) {
  const [isCreateOpen, setIsCreateOpen] = React.useState(false)
  const [isEditOpen, setIsEditOpen] = React.useState(false)
  const [isDeleteOpen, setIsDeleteOpen] = React.useState(false)
  const [contractToDelete, setContractToDelete] = React.useState<string | null>(null)
  const [contractToEdit, setContractToEdit] = React.useState<StaffContractResponse | null>(null)

  const { data, isLoading } = useQuery({
    ...getStaffContractsQueryOptions({
      client: authClient,
    }),
  })

  const createContract = useCreateStaffContract()
  const updateContract = useUpdateStaffContract()
  const deleteContract = useDeleteStaffContract()

  const [formData, setFormData] = React.useState({
    staff_id: staffId ?? '',
    contract_type: 'Permanent',
    start_date: '',
    end_date: '',
    salary_amount: 0,
    currency: 'USD',
    status: 'Active',
  })

  const handleSubmit = () => {
    if (contractToEdit) {
      updateContract.mutate(
        { path: { id: contractToEdit.id }, body: formData },
        {
          onSuccess: () => {
            setIsEditOpen(false)
            setContractToEdit(null)
          },
        },
      )
    } else {
      createContract.mutate(formData, {
        onSuccess: () => {
          setIsCreateOpen(false)
          setFormData({
            staff_id: staffId ?? '',
            contract_type: 'Permanent',
            start_date: '',
            end_date: '',
            salary_amount: 0,
            currency: 'USD',
            status: 'Active',
          })
        },
      })
    }
  }

  const openEdit = (contract: StaffContractResponse) => {
    setContractToEdit(contract)
    setFormData({
      staff_id: contract.staff_id,
      contract_type: contract.contract_type,
      start_date: contract.start_date,
      end_date: contract.end_date ?? '',
      salary_amount: contract.salary_amount ?? 0,
      currency: contract.currency,
      status: contract.status,
    })
    setIsEditOpen(true)
  }

  const handleDelete = () => {
    if (contractToDelete) {
      deleteContract.mutate(
        { id: contractToDelete },
        {
          onSuccess: () => {
            setIsDeleteOpen(false)
            setContractToDelete(null)
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
            <CardTitle>Employment Contracts</CardTitle>
            <CardDescription>
              Manage staff employment contracts and terms
            </CardDescription>
          </Stack>
          <Button size="sm" onClick={() => setIsCreateOpen(true)}>
            <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
            Add Contract
          </Button>
        </HStack>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[400px]">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Contract Type</TableHead>
                <TableHead>Start Date</TableHead>
                <TableHead>End Date</TableHead>
                <TableHead>Salary</TableHead>
                <TableHead>Status</TableHead>
                <TableHead className="w-[100px]">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {isLoading ? (
                <TableRow>
                  <TableCell colSpan={6} className="text-center py-8">
                    <Text muted>Loading...</Text>
                  </TableCell>
                </TableRow>
              ) : !data?.data || data.data.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={6} className="text-center py-8">
                    <Text muted>No contracts found</Text>
                  </TableCell>
                </TableRow>
              ) : (
                data.data.map((contract) => (
                  <TableRow key={contract.id}>
                    <TableCell>{contract.contract_type}</TableCell>
                    <TableCell>
                      {format(new Date(contract.start_date), 'MMM dd, yyyy')}
                    </TableCell>
                    <TableCell>
                      {contract.end_date
                        ? format(new Date(contract.end_date), 'MMM dd, yyyy')
                        : '—'}
                    </TableCell>
                    <TableCell>
                      {contract.salary_amount
                        ? `${contract.currency} ${contract.salary_amount.toLocaleString()}`
                        : '—'}
                    </TableCell>
                    <TableCell>
                      <Badge
                        variant="outline"
                        className={
                          contract.status === 'Active'
                            ? 'bg-green-500/10 text-green-500'
                            : 'bg-gray-500/10 text-gray-500'
                        }
                      >
                        {contract.status}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      <HStack gap={2}>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => openEdit(contract)}
                        >
                          <HugeiconsIcon icon={PencilEdit01Icon} className="size-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => {
                            setContractToDelete(contract.id)
                            setIsDeleteOpen(true)
                          }}
                        >
                          <HugeiconsIcon icon={Delete02Icon} className="size-4" />
                        </Button>
                      </HStack>
                    </TableCell>
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </ScrollArea>
      </CardContent>

      {/* Create/Edit Dialog */}
      <Dialog
        open={isCreateOpen || isEditOpen}
        onOpenChange={(open) => {
          if (!open) {
            setIsCreateOpen(false)
            setIsEditOpen(false)
            setContractToEdit(null)
            setFormData({
              staff_id: staffId ?? '',
              contract_type: 'Permanent',
              start_date: '',
              end_date: '',
              salary_amount: 0,
              currency: 'USD',
              status: 'Active',
            })
          }
        }}
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {contractToEdit ? 'Edit Contract' : 'Add Contract'}
            </DialogTitle>
            <DialogDescription>
              {contractToEdit
                ? 'Update contract information'
                : 'Add new employment contract'}
            </DialogDescription>
          </DialogHeader>
          <Stack gap={4}>
            <div className="grid gap-2">
              <Label htmlFor="contract_type">Contract Type</Label>
              <Input
                id="contract_type"
                value={formData.contract_type}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, contract_type: e.target.value }))
                }
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="start_date">Start Date</Label>
              <Input
                id="start_date"
                type="date"
                value={formData.start_date}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, start_date: e.target.value }))
                }
                required
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="end_date">End Date (if applicable)</Label>
              <Input
                id="end_date"
                type="date"
                value={formData.end_date}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, end_date: e.target.value }))
                }
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="salary">Salary Amount</Label>
              <Input
                id="salary"
                type="number"
                value={formData.salary_amount}
                onChange={(e) =>
                  setFormData((prev) => ({
                    ...prev,
                    salary_amount: Number(e.target.value),
                  }))
                }
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="currency">Currency</Label>
              <Input
                id="currency"
                value={formData.currency}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, currency: e.target.value }))
                }
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="status">Status</Label>
              <Input
                id="status"
                value={formData.status}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, status: e.target.value }))
                }
              />
            </div>
          </Stack>
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => {
                setIsCreateOpen(false)
                setIsEditOpen(false)
                setContractToEdit(null)
              }}
            >
              Cancel
            </Button>
            <Button
              onClick={handleSubmit}
              disabled={createContract.isPending || updateContract.isPending}
            >
              {createContract.isPending || updateContract.isPending
                ? 'Saving...'
                : 'Save'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation */}
      <AlertDialog open={isDeleteOpen} onOpenChange={setIsDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Contract</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this contract? This action cannot
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
