import z from "zod";

export const ErrorFields = z.tuple([z.string(), 
  z.union([
    z.literal("NotFound"),
    z.literal("Conflict"),
    z.object({
      TooLong: z.number(),
    }),
    z.object({
      TooShort: z.number(),
    }),
  ])]).array()
export const LoginRequest =
  z.object({
    email: z.string(),
    password: z.string(),
  })

export const UploadRequest =
  z.object({
    idempotence_token: z.string(),
    idk: z.string(),
  })

export const RegisterRequest =
  z.object({
    idempotence_token: z.string(),
    email: z.string(),
    password: z.string(),
    first_name: z.string(),
    middle_name: z.string().optional(),
    last_name: z.string(),
    code: z.string().optional(),
  })