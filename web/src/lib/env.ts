import { z } from 'zod';

const envSchema = z.object({
    VITE_APP_NAME: z.string().min(1, 'App name is required'),
    VITE_API_URL: z.string().url('API URL must be a valid URL'),
});

const _env = envSchema.safeParse(import.meta.env);

if (!_env.success) {
    console.error('❌ Invalid environment variables:', _env.error.format());
    throw new Error('Invalid environment variables');
}

export const env = _env.data;
