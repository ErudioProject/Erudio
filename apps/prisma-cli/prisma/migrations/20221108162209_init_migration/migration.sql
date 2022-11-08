-- CreateEnum
CREATE TYPE "GrammaticalForm" AS ENUM ('Masculinine', 'Feminine', 'Indeterminate');

-- CreateTable
CREATE TABLE "Session" (
    "userId" TEXT NOT NULL,
    "sessionId" BYTEA NOT NULL,
    "validUntil" TIMESTAMP(3) NOT NULL
);

-- CreateTable
CREATE TABLE "User" (
    "id" TEXT NOT NULL,
    "passwordHash" BYTEA NOT NULL,
    "twoFactorAuth" BOOLEAN NOT NULL,
    "grammaticalForm" "GrammaticalForm" NOT NULL,

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "PiiData" (
    "id" TEXT NOT NULL,
    "userId" TEXT NOT NULL,
    "email" TEXT,
    "pesel" TEXT,
    "birthDate" TIMESTAMP(3),
    "legalName" TEXT,
    "displayName" TEXT,
    "phonePrefix" TEXT,
    "phoneNumber" TEXT,

    CONSTRAINT "PiiData_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Session_sessionId_key" ON "Session"("sessionId");

-- CreateIndex
CREATE INDEX "Session_userId_idx" ON "Session"("userId");

-- CreateIndex
CREATE INDEX "Session_validUntil_idx" ON "Session"("validUntil");

-- CreateIndex
CREATE UNIQUE INDEX "PiiData_userId_key" ON "PiiData"("userId");

-- CreateIndex
CREATE UNIQUE INDEX "PiiData_email_key" ON "PiiData"("email");

-- CreateIndex
CREATE UNIQUE INDEX "PiiData_phoneNumber_key" ON "PiiData"("phoneNumber");

-- AddForeignKey
ALTER TABLE "Session" ADD CONSTRAINT "Session_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "PiiData" ADD CONSTRAINT "PiiData_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;
