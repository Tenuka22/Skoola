import { defineConfig } from '@hey-api/openapi-ts'

export default defineConfig({
  input: 'http://localhost:8080/openapi.json',
  output: 'src/lib/api',
  plugins: [
    'zod',
    '@tanstack/react-query',
    '@hey-api/typescript',
    '@hey-api/sdk',
    {
      name: '@hey-api/schemas',
      type: 'json',
    },
  ],
})
