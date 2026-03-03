'use client'

import * as React from 'react'
import type { Table } from '@tanstack/react-table'

import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from '@/components/ui/pagination'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { cn } from '@/lib/utils'
import { HStack, Text } from '@/components/primitives'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'

interface DataTablePaginationProps<TData> {
  table: Table<TData>
  pageIndex: number
  pageSize: number
  pageCount: number
  canNextPage: boolean
  canPreviousPage: boolean
  fetchNextPage: () => void
  fetchPreviousPage: () => void
  onPageSizeChange?: (pageSize: number) => void
  onPageIndexChange?: (pageIndex: number) => void
  pageSizeOptions?: Array<number>
  isLoading?: boolean
  totalCount: number
}

function ActionTooltip({
  children,
  label,
}: {
  children: React.ReactElement
  label: string
}) {
  return (
    <Tooltip>
      <TooltipTrigger render={children} />
      <TooltipContent>{label}</TooltipContent>
    </Tooltip>
  )
}

export function DataTablePagination<TData>({
  table,
  pageIndex,
  pageSize,
  pageCount,
  canNextPage,
  canPreviousPage,
  fetchNextPage,
  fetchPreviousPage,
  onPageSizeChange,
  onPageIndexChange,
  pageSizeOptions = [10, 20, 30, 40, 50],
  isLoading,
  totalCount,
}: DataTablePaginationProps<TData>) {
  return (
    <TooltipProvider>
      <HStack justify="between" py={4} px={2}>
        <HStack gap={6} className="lg:gap-8">
          <Text
            size="xs"
            muted
            className="font-medium whitespace-nowrap hidden sm:block"
          >
            Showing{' '}
            <Text size="xs" className="text-foreground">
              {pageIndex * pageSize + 1}
            </Text>{' '}
            to{' '}
            <Text size="xs" className="text-foreground">
              {Math.min((pageIndex + 1) * pageSize, totalCount)}
            </Text>{' '}
            of{' '}
            <Text size="xs" className="text-foreground">
              {totalCount}
            </Text>{' '}
            results
          </Text>
          <HStack gap={2}>
            <Text
              size="xs"
              muted
              className="font-medium whitespace-nowrap hidden sm:block"
            >
              Rows per page
            </Text>
            <ActionTooltip label="Select number of rows per page">
              <Select
                value={`${pageSize}`}
                onValueChange={(value) => onPageSizeChange?.(Number(value))}
              >
                <SelectTrigger className="h-8 w-[70px]">
                  <SelectValue placeholder={pageSize} />
                </SelectTrigger>
                <SelectContent side="top">
                  {pageSizeOptions.map((option) => (
                    <SelectItem key={option} value={`${option}`}>
                      {option}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </ActionTooltip>
          </HStack>
        </HStack>

        <Pagination className="mx-0 w-auto justify-end">
          <PaginationContent>
            <PaginationItem>
              <ActionTooltip label="Go to previous page">
                <PaginationPrevious
                  className={cn(
                    !canPreviousPage || isLoading
                      ? 'pointer-events-none opacity-50'
                      : 'cursor-pointer',
                  )}
                  onClick={
                    canPreviousPage && !isLoading
                      ? fetchPreviousPage
                      : undefined
                  }
                />
              </ActionTooltip>
            </PaginationItem>

            <div className="hidden sm:flex items-center">
              {pageCount > 0 &&
                (() => {
                  const currentPage = pageIndex + 1
                  const maxVisiblePages = 5

                  if (pageCount <= maxVisiblePages) {
                    return Array.from({ length: pageCount }).map((_, i) => (
                      <PaginationItem key={i}>
                        <PaginationLink
                          isActive={currentPage === i + 1}
                          onClick={() =>
                            onPageIndexChange
                              ? onPageIndexChange(i)
                              : table.setPageIndex(i)
                          }
                          className="cursor-pointer"
                        >
                          {i + 1}
                        </PaginationLink>
                      </PaginationItem>
                    ))
                  }

                  const pages: Array<React.ReactNode> = []
                  pages.push(
                    <PaginationItem key={0}>
                      <PaginationLink
                        isActive={currentPage === 1}
                        onClick={() =>
                          onPageIndexChange
                            ? onPageIndexChange(0)
                            : table.setPageIndex(0)
                        }
                        className="cursor-pointer"
                      >
                        1
                      </PaginationLink>
                    </PaginationItem>,
                  )

                  if (currentPage > 3) {
                    pages.push(
                      <PaginationItem key="ellipsis-start">
                        <PaginationEllipsis />
                      </PaginationItem>,
                    )
                  }

                  const startPage = Math.max(2, currentPage - 1)
                  const endPage = Math.min(pageCount - 1, currentPage + 1)

                  for (let i = startPage; i <= endPage; i++) {
                    pages.push(
                      <PaginationItem key={i - 1}>
                        <PaginationLink
                          isActive={currentPage === i}
                          onClick={() =>
                            onPageIndexChange
                              ? onPageIndexChange(i - 1)
                              : table.setPageIndex(i - 1)
                          }
                          className="cursor-pointer"
                        >
                          {i}
                        </PaginationLink>
                      </PaginationItem>,
                    )
                  }

                  if (currentPage < pageCount - 2) {
                    pages.push(
                      <PaginationItem key="ellipsis-end">
                        <PaginationEllipsis />
                      </PaginationItem>,
                    )
                  }

                  pages.push(
                    <PaginationItem key={pageCount - 1}>
                      <PaginationLink
                        isActive={currentPage === pageCount}
                        onClick={() =>
                          onPageIndexChange
                            ? onPageIndexChange(pageCount - 1)
                            : table.setPageIndex(pageCount - 1)
                        }
                        className="cursor-pointer"
                      >
                        {pageCount}
                      </PaginationLink>
                    </PaginationItem>,
                  )

                  return pages
                })()}
            </div>
            <PaginationItem className="sm:hidden">
              <span className="text-sm px-4">
                Page {pageIndex + 1} of {Math.max(pageCount, 1)}
              </span>
            </PaginationItem>

            <PaginationItem>
              <ActionTooltip label="Go to next page">
                <PaginationNext
                  className={cn(
                    !canNextPage || isLoading
                      ? 'pointer-events-none opacity-50'
                      : 'cursor-pointer',
                  )}
                  onClick={
                    canNextPage && !isLoading ? fetchNextPage : undefined
                  }
                />
              </ActionTooltip>
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      </HStack>
    </TooltipProvider>
  )
}
