import {
  Add01Icon,
  Download01Icon,
  LayoutGridIcon,
  ListViewIcon,
  Search01Icon,
  TableIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useUsersStore } from '../store'
import { Input } from '@/components/ui/input'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Button } from '@/components/ui/button'

interface UsersToolbarProps {
  handleExportCSV: () => void
}

export function UsersToolbar({ handleExportCSV }: UsersToolbarProps) {
  const {
    view,
    setView,
    search,
    setSearch,
    setIsCreateUserOpen,
  } = useUsersStore()

  return (
    <div className="mb-4 flex flex-col gap-4 px-8 sm:flex-row sm:items-center sm:justify-between">
      <Tabs value={view} onValueChange={(value: any) => setView(value)}>
        <TabsList className="h-9 w-fit rounded-lg bg-muted/50 p-1">
          <TabsTrigger
            value="table"
            className="flex h-7 gap-2 rounded-md px-3 text-xs data-[state=active]:bg-background data-[state=active]:shadow-sm"
          >
            <HugeiconsIcon icon={TableIcon} className="size-3.5" />
            Table
          </TabsTrigger>
          <TabsTrigger
            value="board"
            className="flex h-7 gap-2 rounded-md px-3 text-xs data-[state=active]:bg-background data-[state=active]:shadow-sm"
          >
            <HugeiconsIcon icon={LayoutGridIcon} className="size-3.5" />
            Board
          </TabsTrigger>
          <TabsTrigger
            value="list"
            className="flex h-7 gap-2 rounded-md px-3 text-xs data-[state=active]:bg-background data-[state=active]:shadow-sm"
          >
            <HugeiconsIcon icon={ListViewIcon} className="size-3.5" />
            List
          </TabsTrigger>
        </TabsList>
      </Tabs>

      <div className="flex items-center gap-2 overflow-x-auto pb-2 sm:w-auto sm:pb-0">
        <div className="group relative min-w-[200px]">
          <HugeiconsIcon
            icon={Search01Icon}
            className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground transition-colors group-focus-within:text-foreground"
          />
          <Input
            placeholder="Search"
            className="h-9 w-full border-border/60 bg-background pl-9 transition-all focus-visible:border-primary focus-visible:ring-1 focus-visible:ring-primary sm:w-64"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>


        <Button
          variant="outline"
          size="sm"
          className="h-9 gap-2 text-muted-foreground hover:text-foreground"
          onClick={handleExportCSV}
        >
          <HugeiconsIcon icon={Download01Icon} className="size-4" />
          Export
        </Button>

        <Button
          size="sm"
          className="h-9 gap-2 bg-primary text-primary-foreground shadow-sm hover:bg-primary/90"
          onClick={() => setIsCreateUserOpen(true)}
        >
          Add User
          <HugeiconsIcon icon={Add01Icon} className="size-4" />
        </Button>
      </div>
    </div>
  )
}
