//  @ts-check

import { tanstackConfig } from '@tanstack/eslint-config'

import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import tsEslint from '@typescript-eslint/eslint-plugin'

export default [
  {
    ignores: ['.output/**', 'src/lib/api/**/*.gen.ts'],
  },
  ...tanstackConfig,
  {
    files: ['**/*.{js,ts,jsx,tsx}'],
    plugins: {
      'react-hooks': reactHooks,
      '@typescript-eslint': tsEslint,
    },
    rules: {
      'react-hooks/rules-of-hooks': 'error',
      'react-hooks/exhaustive-deps': 'warn',
      '@typescript-eslint/no-use-before-define': 'off',
      '@typescript-eslint/naming-convention': 'off',
      '@typescript-eslint/no-unnecessary-condition': 'off',
      '@typescript-eslint/require-await': 'off',
      '@typescript-eslint/ban-ts-comment': 'off',
      'no-shadow': 'off',
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-unsafe-assignment': 'error',
      '@typescript-eslint/no-unsafe-member-access': 'error',
      '@typescript-eslint/no-unsafe-call': 'error',
      '@typescript-eslint/no-unsafe-return': 'error',
      '@typescript-eslint/no-unsafe-argument': 'error',
      '@typescript-eslint/consistent-type-assertions': [
        'error',
        { assertionStyle: 'never' },
      ],
      '@typescript-eslint/no-non-null-assertion': 'error',
    },
    parserOptions: {
      project: true,
      tsconfigRootDir: import.meta.dirname,
    },
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
  },
]
