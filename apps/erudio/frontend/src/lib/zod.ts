import z from "zod";

export const Pagination =
  z.object({
    skip: z.number(),
    take: z.number(),
  })

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

export const AddSchoolRequest =
  z.object({
    idempotence_token: z.string(),
    name: z.string(),
  })

export const UpdateSchoolRequest =
  z.object({
    idempotence_token: z.string(),
    id: z.string(),
    name: z.string().optional(),
  })

export const SearchSchoolsRequest =
  z.object({
    page: Pagination.optional(),
    name: z.string(),
  })

export const GetSchoolRequest =
  z.object({
    id: z.string(),
  })

export const AddUserToSchoolRequest =
  z.object({
    idempotence_token: z.string(),
    school_id: z.string(),
    user_id: z.string(),
    relation_type: SchoolRelationType,
  })

export const GetUserRequest =
  z.object({
    id: z.string(),
    school_id: z.string().optional(),
  })

export const SearchUsersRequest =
  z.object({
    page: Pagination.optional(),
    school_id: z.string(),
    query: z.string(),
  })
