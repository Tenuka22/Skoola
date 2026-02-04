import React from 'react';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Label } from '@/components/ui/label';

interface AssignRoleDialogProps {
  staffId: string;
  currentRole: string;
  // TODO: Define props for role assignment handler
}

export const AssignRoleDialog = ({ staffId, currentRole /* props */ }: AssignRoleDialogProps) => {
  // Placeholder for roles list
  const roles = ["Admin", "Teacher", "Librarian", "Accountant"];
  const [selectedRole, setSelectedRole] = React.useState(currentRole);

  return (
    <Dialog>
      <DialogTrigger asChild={true}>
        <Button variant="outline">Assign Role</Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Assign Role to Staff</DialogTitle>
          <DialogDescription>
            Select a new role for the staff member.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="staffName" className="text-right">Staff Name</Label>
            <Input id="staffName" value={staffId} className="col-span-3" disabled />
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="role" className="text-right">Role</Label>
            <Select value={selectedRole} onValueChange={(value) => setSelectedRole(value || '')}>
              <SelectTrigger id="role" className="col-span-3">
                <SelectValue placeholder="Select a role" />
              </SelectTrigger>
              <SelectContent>
                {roles.map((role) => (
                  <SelectItem key={role} value={role}>{role}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </div>
        {/* DialogFooter would go here for save/cancel buttons */}
      </DialogContent>
    </Dialog>
  );
};
