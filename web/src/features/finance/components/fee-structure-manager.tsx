import { useState } from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import { PlusSignIcon, Delete02Icon } from '@hugeicons/core-free-icons'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'

type FeeStructureManagerProps = object
// TODO: Define props for data and actions

interface FeeRule {
  id: string
  grade: string
  feeType: string
  amount: number
}

export const FeeStructureManager = (_props: FeeStructureManagerProps) => {
  const [feeRules, setFeeRules] = useState<Array<FeeRule>>([
    { id: 'fr1', grade: 'Grade 1', feeType: 'Tuition', amount: 5000 },
    { id: 'fr2', grade: 'Grade 1', feeType: 'Library', amount: 200 },
    { id: 'fr3', grade: 'Grade 2', feeType: 'Tuition', amount: 5500 },
  ])

  const [newRule, setNewRule] = useState<Omit<FeeRule, 'id'>>({
    grade: '',
    feeType: '',
    amount: 0,
  })

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { id, value } = e.target
    setNewRule((prev) => ({
      ...prev,
      [id]: id === 'amount' ? parseFloat(value) || 0 : value,
    }))
  }

  const addFeeRule = () => {
    if (newRule.grade && newRule.feeType && newRule.amount > 0) {
      setFeeRules((prev) => [
        ...prev,
        { ...newRule, id: Date.now().toString() },
      ])
      setNewRule({ grade: '', feeType: '', amount: 0 })
    } else {
      alert('Please fill all fields for the new fee rule.')
    }
  }

  const removeFeeRule = (id: string) => {
    setFeeRules((prev) => prev.filter((rule) => rule.id !== id))
  }

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold">Fee Structure Management</h2>

      <Card>
        <CardHeader>
          <CardTitle>Define Fee Rules</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid gap-4 mb-6">
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="grade">Grade</Label>
              <Input
                id="grade"
                value={newRule.grade}
                onChange={handleInputChange}
                className="col-span-3"
                placeholder="e.g., Grade 1"
              />
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="feeType">Fee Type</Label>
              <Input
                id="feeType"
                value={newRule.feeType}
                onChange={handleInputChange}
                className="col-span-3"
                placeholder="e.g., Tuition, Library"
              />
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="amount">Amount</Label>
              <Input
                id="amount"
                type="number"
                value={newRule.amount}
                onChange={handleInputChange}
                className="col-span-3"
              />
            </div>
            <Button onClick={addFeeRule} className="w-fit">
              <HugeiconsIcon icon={PlusSignIcon} className="h-4 w-4 mr-2" />
              Add Rule
            </Button>
          </div>

          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Grade</TableHead>
                <TableHead>Fee Type</TableHead>
                <TableHead className="text-right">Amount</TableHead>
                <TableHead className="text-right">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {feeRules.map((rule) => (
                <TableRow key={rule.id}>
                  <TableCell>{rule.grade}</TableCell>
                  <TableCell>{rule.feeType}</TableCell>
                  <TableCell className="text-right">
                    {rule.amount.toFixed(2)}
                  </TableCell>
                  <TableCell className="text-right">
                    <Button
                      variant="destructive"
                      size="icon"
                      onClick={() => removeFeeRule(rule.id)}
                    >
                      <HugeiconsIcon icon={Delete02Icon} className="h-4 w-4" />
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
  )
}
