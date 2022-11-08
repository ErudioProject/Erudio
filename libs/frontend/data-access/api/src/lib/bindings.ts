// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "public.login", input: LoginRequest, result: LoginResponse } | 
        { key: "public.register", input: RegisterRequest, result: null } | 
        { key: "public.version", input: never, result: string } | 
        { key: "user.me", input: never, result: { two_factor_auth: boolean, grammatical_form: GrammaticalForm, pii_data: PiiData | null } },
    mutations: never,
    subscriptions: never
};

export type GrammaticalForm = "Masculinine" | "Feminine" | "Indeterminate"

export interface LoginRequest { email: string, password: string }

export type LoginResponse = { t: "Success" } | { t: "TwoFactorAuth", c: TwoFactorAuthType }

export interface PiiData { id: string, userId: string, email: string | null, pesel: string | null, birthDate: string | null, legalName: string | null, displayName: string | null, phonePrefix: string | null, phoneNumber: string | null }

export interface RegisterRequest { email: string, password: string, code: null }

export type TwoFactorAuthType = "GoogleAuth" | "Sms" | "EMail"
