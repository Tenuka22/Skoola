import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Button } from '@/components/ui/button';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';


interface FeeCollectionPosProps {
  // TODO: Define props for student lookup, fee rules, payment processing
}

interface LineItem {
  id: string;
  description: string;
  amount: number;
  quantity: number;
}

export const FeeCollectionPos = ({ /* props */ }: FeeCollectionPosProps) => {
  const [studentId, setStudentId] = useState('');
  const [studentName, setStudentName] = useState('');
  const [lineItems, setLineItems] = useState<LineItem[]>([]);
  const [paymentAmount, setPaymentAmount] = useState(0);

  // Placeholder for searching student and fetching fee details
  const handleStudentSearch = () => {
    if (studentId) {
      // Mock student data and fee items
      setStudentName(`Student: ${studentId}`);
      setLineItems([
        { id: 'fee1', description: 'Tuition Fee (Term 1)', amount: 5000, quantity: 1 },
        { id: 'fee2', description: 'Library Fee', amount: 200, quantity: 1 },
      ]);
    } else {
      setStudentName('');
      setLineItems([]);
    }
  };

  const subtotal = lineItems.reduce((sum, item) => sum + item.amount * item.quantity, 0);
  const totalDue = subtotal; // No taxes/discounts for now
  const changeDue = paymentAmount > totalDue ? paymentAmount - totalDue : 0;

  const handleProcessPayment = () => {
    if (paymentAmount >= totalDue && studentId && lineItems.length > 0) {
      alert(`Payment of ${paymentAmount} processed for ${studentName}. Change due: ${changeDue.toFixed(2)}`);
      // TODO: Implement actual payment processing and clear state
      setStudentId('');
      setStudentName('');
      setLineItems([]);
      setPaymentAmount(0);
    } else {
      alert('Please ensure a student is selected, items are added, and payment covers the total due.');
    }
  };

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold">Fee Collection Point-of-Sale</h2>

      <Card>
        <CardHeader>
          <CardTitle>Student & Fees</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <div className="flex gap-4 items-end">
            <div className="grid gap-2 flex-grow">
              <Label htmlFor="studentId">Student ID</Label>
              <Input
                id="studentId"
                value={studentId}
                onChange={(e) => setStudentId(e.target.value)}
                placeholder="Enter Student ID"
              />
            </div>
            <Button onClick={handleStudentSearch}>Search Student</Button>
          </div>
          {studentName && <p className="text-lg font-medium">{studentName}</p>}

          {lineItems.length > 0 && (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Description</TableHead>
                  <TableHead className="text-right">Amount</TableHead>
                  <TableHead className="text-right">Qty</TableHead>
                  <TableHead className="text-right">Total</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {lineItems.map(item => (
                  <TableRow key={item.id}>
                    <TableCell>{item.description}</TableCell>
                    <TableCell className="text-right">{item.amount.toFixed(2)}</TableCell>
                    <TableCell className="text-right">{item.quantity}</TableCell>
                    <TableCell className="text-right">{(item.amount * item.quantity).toFixed(2)}</TableCell>
                  </TableRow>
                ))}
                <TableRow>
                  <TableCell colSpan={3} className="text-right font-bold">Subtotal</TableCell>
                  <TableCell className="text-right font-bold">{subtotal.toFixed(2)}</TableCell>
                </TableRow>
                <TableRow>
                  <TableCell colSpan={3} className="text-right font-bold">Total Due</TableCell>
                  <TableCell className="text-right font-bold">{totalDue.toFixed(2)}</TableCell>
                </TableRow>
              </TableBody>
            </Table>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Payment</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <div className="grid grid-cols-2 items-center gap-4">
            <Label htmlFor="paymentAmount">Payment Amount</Label>
            <Input
              id="paymentAmount"
              type="number"
              value={paymentAmount}
              onChange={(e) => setPaymentAmount(parseFloat(e.target.value) || 0)}
              className="text-right"
            />
          </div>
          <div className="grid grid-cols-2 items-center gap-4">
            <Label>Change Due</Label>
            <p className="text-right font-bold">{changeDue.toFixed(2)}</p>
          </div>
          <Button onClick={handleProcessPayment} className="w-full">
            Process Payment
          </Button>
        </CardContent>
      </Card>
    </div>
  );
};
