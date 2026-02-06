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
    },
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
  },
]
