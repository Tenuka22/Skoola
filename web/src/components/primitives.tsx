import * as React from 'react'
import { cva } from 'class-variance-authority'
import type { VariantProps } from 'class-variance-authority'
import { cn } from '@/lib/utils'

export type Spacing =
  | 0
  | 1
  | 2
  | 3
  | 4
  | 5
  | 6
  | 8
  | 10
  | 12
  | 16
  | 20
  | 24
  | 32
  | 40
  | 48
  | 56
  | 64

const gapMap: Record<Spacing, string> = {
  0: 'gap-0',
  1: 'gap-1',
  2: 'gap-2',
  3: 'gap-3',
  4: 'gap-4',
  5: 'gap-5',
  6: 'gap-6',
  8: 'gap-8',
  10: 'gap-10',
  12: 'gap-12',
  16: 'gap-16',
  20: 'gap-20',
  24: 'gap-24',
  32: 'gap-32',
  40: 'gap-40',
  48: 'gap-48',
  56: 'gap-56',
  64: 'gap-64',
}

const pMap: Record<Spacing, string> = {
  0: 'p-0',
  1: 'p-1',
  2: 'p-2',
  3: 'p-3',
  4: 'p-4',
  5: 'p-5',
  6: 'p-6',
  8: 'p-8',
  10: 'p-10',
  12: 'p-12',
  16: 'p-16',
  20: 'p-20',
  24: 'p-24',
  32: 'p-32',
  40: 'p-40',
  48: 'p-48',
  56: 'p-56',
  64: 'p-64',
}

const pxMap: Record<Spacing, string> = {
  0: 'px-0',
  1: 'px-1',
  2: 'px-2',
  3: 'px-3',
  4: 'px-4',
  5: 'px-5',
  6: 'px-6',
  8: 'px-8',
  10: 'px-10',
  12: 'px-12',
  16: 'px-16',
  20: 'px-20',
  24: 'px-24',
  32: 'px-32',
  40: 'px-40',
  48: 'px-48',
  56: 'px-56',
  64: 'px-64',
}

const pyMap: Record<Spacing, string> = {
  0: 'py-0',
  1: 'py-1',
  2: 'py-2',
  3: 'py-3',
  4: 'py-4',
  5: 'py-5',
  6: 'py-6',
  8: 'py-8',
  10: 'py-10',
  12: 'py-12',
  16: 'py-16',
  20: 'py-20',
  24: 'py-24',
  32: 'py-32',
  40: 'py-40',
  48: 'py-48',
  56: 'py-56',
  64: 'py-64',
}

export interface BoxProps extends React.HTMLAttributes<HTMLDivElement> {
  p?: Spacing
  px?: Spacing
  py?: Spacing
  rounded?: 'none' | 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '3xl' | 'full'
  bg?: string
  as?: React.ElementType
}

export const Box = React.forwardRef<HTMLDivElement, BoxProps>(
  (
    { className, p, px, py, rounded, bg, as: Component = 'div', ...props },
    ref,
  ) => {
    const roundedClass = rounded
      ? rounded === 'full'
        ? 'rounded-full'
        : `rounded-${rounded}`
      : ''
    return (
      <Component
        ref={ref}
        className={cn(
          p !== undefined && pMap[p],
          px !== undefined && pxMap[px],
          py !== undefined && pyMap[py],
          roundedClass,
          bg,
          className,
        )}
        {...props}
      />
    )
  },
)
Box.displayName = 'Box'

interface StackProps extends BoxProps {
  gap?: Spacing
  align?: 'start' | 'center' | 'end' | 'baseline' | 'stretch'
  justify?: 'start' | 'center' | 'end' | 'between' | 'around' | 'evenly'
}

export const Stack = React.forwardRef<HTMLDivElement, StackProps>(
  ({ className, gap = 4, align, justify, ...props }, ref) => {
    const alignClass = align
      ? {
          start: 'items-start',
          center: 'items-center',
          end: 'items-end',
          baseline: 'items-baseline',
          stretch: 'items-stretch',
        }[align]
      : ''

    const justifyClass = justify
      ? {
          start: 'justify-start',
          center: 'justify-center',
          end: 'justify-end',
          between: 'justify-between',
          around: 'justify-around',
          evenly: 'justify-evenly',
        }[justify]
      : ''

    return (
      <Box
        ref={ref}
        className={cn(
          'flex flex-col',
          gap !== undefined && gapMap[gap],
          alignClass,
          justifyClass,
          className,
        )}
        {...props}
      />
    )
  },
)
Stack.displayName = 'Stack'

interface HStackProps extends StackProps {}

export const HStack = React.forwardRef<HTMLDivElement, HStackProps>(
  ({ className, gap = 4, align = 'center', justify, ...props }, ref) => {
    const alignClass = {
      start: 'items-start',
      center: 'items-center',
      end: 'items-end',
      baseline: 'items-baseline',
      stretch: 'items-stretch',
    }[align]

    const justifyClass = justify
      ? {
          start: 'justify-start',
          center: 'justify-center',
          end: 'justify-end',
          between: 'justify-between',
          around: 'justify-around',
          evenly: 'justify-evenly',
        }[justify]
      : ''

    return (
      <Box
        ref={ref}
        className={cn(
          'flex flex-row',
          alignClass,
          justifyClass,
          gap !== undefined && gapMap[gap],
          className,
        )}
        {...props}
      />
    )
  },
)
HStack.displayName = 'HStack'

const textVariants = cva('', {
  variants: {
    size: {
      xs: 'text-xs',
      sm: 'text-sm',
      base: 'text-base',
      lg: 'text-lg',
      xl: 'text-xl',
      '2xl': 'text-2xl',
    },
    muted: {
      true: 'text-muted-foreground',
      false: '',
    },
  },
  defaultVariants: {
    size: 'base',
    muted: false,
  },
})

interface TextProps
  extends
    React.HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof textVariants> {
  as?: 'span' | 'p' | 'div'
}

export const Text = React.forwardRef<HTMLDivElement, TextProps>(
  ({ className, size, muted, as: Component = 'span', ...props }, ref) => (
    <Component
      ref={ref}
      className={cn(textVariants({ size, muted }), className)}
      {...props}
    />
  ),
)
Text.displayName = 'Text'
Text.displayName = 'Text'

const headingVariants = cva('font-bold tracking-tight text-zinc-100', {
  variants: {
    size: {
      h1: 'text-4xl',
      h2: 'text-3xl',
      h3: 'text-2xl',
      h4: 'text-xl',
    },
  },
  defaultVariants: {
    size: 'h1',
  },
})

interface HeadingProps
  extends
    React.HTMLAttributes<HTMLHeadingElement>,
    VariantProps<typeof headingVariants> {
  as?: 'h1' | 'h2' | 'h3' | 'h4'
}

export const Heading = React.forwardRef<HTMLHeadingElement, HeadingProps>(
  ({ className, size, as: Component = 'h1', ...props }, ref) => (
    <Component
      ref={ref}
      className={cn(headingVariants({ size: size || Component }), className)}
      {...props}
    />
  ),
)
Heading.displayName = 'Heading'

interface GridProps extends BoxProps {
  cols?: 1 | 2 | 3 | 4 | 5 | 6 | 8 | 12
  gap?: Spacing
}

export const Grid = React.forwardRef<HTMLDivElement, GridProps>(
  ({ className, cols = 1, gap = 4, ...props }, ref) => {
    const colClass = (() => {
      const map: Record<number, string> = {}

      for (let i = 1; i <= 12; i++) {
        map[i] = [
          'grid-cols-1',
          `sm:grid-cols-${Math.min(i, 2)}`,
          `md:grid-cols-${Math.min(i, 3)}`,
          `lg:grid-cols-${Math.min(i, 3)}`,
          `xl:grid-cols-${i}`,
        ].join(' ')
      }

      return map
    })()[cols]

    return (
      <Box
        ref={ref}
        className={cn(
          'grid',
          colClass,
          gap !== undefined && gapMap[gap],
          className,
        )}
        {...props}
      />
    )
  },
)
Grid.displayName = 'Grid'

interface ContainerProps extends BoxProps {
  maxWidth?:
    | 'sm'
    | 'md'
    | 'lg'
    | 'xl'
    | '2xl'
    | '3xl'
    | '4xl'
    | '5xl'
    | '6xl'
    | '7xl'
    | 'full'
}

export const Container = React.forwardRef<HTMLDivElement, ContainerProps>(
  ({ className, maxWidth = '7xl', ...props }, ref) => {
    const maxWClass = maxWidth === 'full' ? 'max-w-full' : `max-w-${maxWidth}`
    return (
      <div
        ref={ref}
        className={cn(
          'mx-auto w-full px-4 sm:px-6 lg:px-8',
          maxWClass,
          className,
        )}
        {...props}
      />
    )
  },
)
Container.displayName = 'Container'
