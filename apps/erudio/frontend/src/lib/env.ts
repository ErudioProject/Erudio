import { z } from "zod";

const envSchema = z.object({
    VITE_API_URL: z.string().url()
})

const clientEnv = envSchema.parse(import.meta.env);

export default clientEnv;
