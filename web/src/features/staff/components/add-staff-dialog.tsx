import React from 'react';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger, DialogFooter } from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { CreateStaffRequest, StaffType, EmploymentStatus } from '@/lib/api/types.gen';
import { zCreateStaffRequest } from '@/lib/api/zod.gen';
import { useMutation } from '@tanstack/react-query';
import { postStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Mutation } from '@/lib/api/@tanstack/react-query.gen';
import { toast } from 'sonner';

interface AddStaffDialogProps {
  onStaffAdded?: () => void;
}

export const AddStaffDialog = ({ onStaffAdded }: AddStaffDialogProps) => {
  const [isOpen, setIsOpen] = React.useState(false);

  const form = useForm<CreateStaffRequest>({
    resolver: zodResolver(zCreateStaffRequest),
    defaultValues: {
      name: '',
      email: '',
      employee_id: '',
      phone: '',
      address: '',
      nic: '',
      dob: new Date(),
      gender: '',
      staff_type: StaffType.Teaching, // Default value
      employment_status: EmploymentStatus.Permanent, // Default value
    },
  });

  const createStaffMutation = useMutation({
    mutationFn: postStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Mutation,
    onSuccess: () => {
      toast.success("Staff member added successfully!");
      form.reset();
      setIsOpen(false);
      onStaffAdded?.();
    },
    onError: (error) => {
      toast.error("Failed to add staff member.", { description: error.message });
    },
  });

  const onSubmit = (data: CreateStaffRequest) => {
    createStaffMutation.mutate({ body: data });
  };

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild={true}>
        <Button>Add New Staff</Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Add Staff Member</DialogTitle>
          <DialogDescription>
            Fill in the details for the new staff member.
          </DialogDescription>
        </DialogHeader>
        <form onSubmit={form.handleSubmit(onSubmit)} className="grid gap-4 py-4">
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="name" className="text-right">Name</Label>
            <Input id="name" {...form.register("name")} className="col-span-3" />
            {form.formState.errors.name && <p className="col-span-4 text-right text-red-500 text-sm">{form.formState.errors.name.message}</p>}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="email" className="text-right">Email</Label>
            <Input id="email" {...form.register("email")} className="col-span-3" type="email" />
            {form.formState.errors.email && <p className="col-span-4 text-right text-red-500 text-sm">{form.formState.errors.email.message}</p>}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="employee_id" className="text-right">Employee ID</Label>
            <Input id="employee_id" {...form.register("employee_id")} className="col-span-3" />
            {form.formState.errors.employee_id && <p className="col-span-4 text-right text-red-500 text-sm">{form.formState.errors.employee_id.message}</p>}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="phone" className="text-right">Phone</Label>
            <Input id="phone" {...form.register("phone")} className="col-span-3" />
            {form.formState.errors.phone && <p className="col-span-4 text-right text-red-500 text-sm">{form.formState.errors.phone.message}</p>}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="address" className="text-right">Address</Label>
            <Input id="address" {...form.register("address")} className="col-span-3" />
            {form.formState.errors.address && <p className="col-span-4 text-right text-red-500 text-sm">{form.formState.errors.address.message}</p>}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="nic" className="text-right">NIC</Label>
            <Input id="nic" {...form.register("nic")} className="col-span-3" />
            {form.formState.errors.nic && <p className="col-span-4 text-right text-red-500 text-sm">{form.formState.errors.nic.message}</p>}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="dob" className="text-right">Date of Birth</Label>
            <Input id="dob" type="date" {...form.register("dob", { valueAsDate: true })} className="col-span-3" />
            {form.formState.errors.dob && <p className="col-span-4 text-right text-red-500 text-sm">{form.formState.errors.dob.message}</p>}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="gender" className="text-right">Gender</Label>
            <Select onValueChange={(value) => form.setValue("gender", value)} defaultValue={form.watch("gender")}>
              <SelectTrigger className="col-span-3" id="gender">
                <SelectValue placeholder="Select Gender" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="Male">Male</SelectItem>
                <SelectItem value="Female">Female</SelectItem>
                <SelectItem value="Other">Other</SelectItem>
              </SelectContent>
            </Select>
            {form.formState.errors.gender && <p className="col-span-4 text-right text-red-500 text-sm">{form.formState.errors.gender.message}</p>}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="staff_type" className="text-right">Staff Type</Label>
            <Select onValueChange={(value: StaffType) => form.setValue("staff_type", value)} defaultValue={form.watch("staff_type")}>
              <SelectTrigger className="col-span-3" id="staff_type">
                <SelectValue placeholder="Select Staff Type" />
              </SelectTrigger>
              <SelectContent>
                {Object.values(StaffType).map((type) => (
                  <SelectItem key={type} value={type}>
                    {type}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.staff_type && <p className="col-span-4 text-right text-red-500 text-sm">{form.formState.errors.staff_type.message}</p>}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="employment_status" className="text-right">Employment Status</Label>
            <Select onValueChange={(value: EmploymentStatus) => form.setValue("employment_status", value)} defaultValue={form.watch("employment_status")}>
              <SelectTrigger className="col-span-3" id="employment_status">
                <SelectValue placeholder="Select Employment Status" />
              </SelectTrigger>
              <SelectContent>
                {Object.values(EmploymentStatus).map((status) => (
                  <SelectItem key={status} value={status}>
                    {status}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.employment_status && <p className="col-span-4 text-right text-red-500 text-sm">{form.formState.errors.employment_status.message}</p>}
          </div>

          <DialogFooter>
            <Button type="submit" disabled={createStaffMutation.isPending}>
              {createStaffMutation.isPending ? "Adding..." : "Add Staff"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};
