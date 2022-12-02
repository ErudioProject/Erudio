/*
  Warnings:

  - A unique constraint covering the columns `[phone_prefix,phone_number]` on the table `pii_data` will be added. If there are existing duplicate values, this will fail.
  - Made the column `legal_name` on table `pii_data` required. This step will fail if there are existing NULL values in that column.
  - Made the column `display_name` on table `pii_data` required. This step will fail if there are existing NULL values in that column.

*/
-- DropIndex
DROP INDEX "pii_data_phone_number_key";

-- AlterTable
ALTER TABLE "pii_data" ALTER COLUMN "legal_name" SET NOT NULL,
ALTER COLUMN "display_name" SET NOT NULL;

-- AlterTable
ALTER TABLE "user" ALTER COLUMN "password_hash" SET DATA TYPE TEXT;

-- CreateIndex
CREATE UNIQUE INDEX "pii_data_phone_prefix_phone_number_key" ON "pii_data"("phone_prefix", "phone_number");
