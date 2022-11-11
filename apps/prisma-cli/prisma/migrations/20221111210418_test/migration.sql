/*
  Warnings:

  - You are about to drop the `PiiData` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `Session` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `User` table. If the table is not empty, all the data it contains will be lost.

*/
-- CreateEnum
CREATE TYPE "grammatical_form" AS ENUM ('masculinine', 'feminine', 'indeterminate');

-- CreateEnum
CREATE TYPE "school_relation_type" AS ENUM ('student', 'teacher', 'admin', 'director');

-- CreateEnum
CREATE TYPE "mark_type" AS ENUM ('no_mark', 'mark', 'points', 'custom');

-- DropForeignKey
ALTER TABLE "PiiData" DROP CONSTRAINT "PiiData_userId_fkey";

-- DropForeignKey
ALTER TABLE "Session" DROP CONSTRAINT "Session_userId_fkey";

-- DropTable
DROP TABLE "PiiData";

-- DropTable
DROP TABLE "Session";

-- DropTable
DROP TABLE "User";

-- DropEnum
DROP TYPE "GrammaticalForm";

-- CreateTable
CREATE TABLE "session" (
    "user_id" UUID NOT NULL,
    "session_id" BYTEA NOT NULL,
    "valid_until" TIMESTAMP(3) NOT NULL
);

-- CreateTable
CREATE TABLE "pii_data" (
    "id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "grammatical_form" "grammatical_form" NOT NULL,
    "email" TEXT,
    "pesel" TEXT,
    "birth_date" TIMESTAMP(3),
    "legal_name" TEXT,
    "display_name" TEXT,
    "phone_prefix" TEXT,
    "phone_number" TEXT,

    CONSTRAINT "pii_data_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "user" (
    "id" UUID NOT NULL,
    "password_hash" BYTEA NOT NULL,
    "two_factor_auth" BOOLEAN NOT NULL,

    CONSTRAINT "user_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "user_school_relation" (
    "user_id" UUID NOT NULL,
    "school_id" UUID NOT NULL,
    "school_relation_type" "school_relation_type" NOT NULL,

    CONSTRAINT "user_school_relation_pkey" PRIMARY KEY ("user_id","school_id")
);

-- CreateTable
CREATE TABLE "user_classes_at_school_relation" (
    "user_id" UUID NOT NULL,
    "school_id" UUID NOT NULL,
    "class_id" UUID NOT NULL,

    CONSTRAINT "user_classes_at_school_relation_pkey" PRIMARY KEY ("user_id","school_id","class_id")
);

-- CreateTable
CREATE TABLE "user_class_admin_at_school_relation" (
    "user_id" UUID NOT NULL,
    "school_id" UUID NOT NULL,
    "class_id" UUID NOT NULL,

    CONSTRAINT "user_class_admin_at_school_relation_pkey" PRIMARY KEY ("user_id","school_id","class_id")
);

-- CreateTable
CREATE TABLE "user_lesson_teached_at_school_relation" (
    "user_id" UUID NOT NULL,
    "school_id" UUID NOT NULL,
    "lesson_id" UUID NOT NULL,

    CONSTRAINT "user_lesson_teached_at_school_relation_pkey" PRIMARY KEY ("user_id","school_id","lesson_id")
);

-- CreateTable
CREATE TABLE "user_lesson_teached_substitute_at_school_relation" (
    "user_id" UUID NOT NULL,
    "school_id" UUID NOT NULL,
    "lesson_id" UUID NOT NULL,

    CONSTRAINT "user_lesson_teached_substitute_at_school_relation_pkey" PRIMARY KEY ("user_id","school_id","lesson_id")
);

-- CreateTable
CREATE TABLE "user_lesson_external_student_at_school_relation" (
    "user_id" UUID NOT NULL,
    "school_id" UUID NOT NULL,
    "lesson_id" UUID NOT NULL,

    CONSTRAINT "user_lesson_external_student_at_school_relation_pkey" PRIMARY KEY ("user_id","school_id","lesson_id")
);

-- CreateTable
CREATE TABLE "school" (
    "id" UUID NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "school_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "school_settings" (
    "id" UUID NOT NULL,
    "school_id" UUID NOT NULL,

    CONSTRAINT "school_settings_pkey" PRIMARY KEY ("id","school_id")
);

-- CreateTable
CREATE TABLE "subject_class_teacher_relation" (
    "school_id" UUID NOT NULL,
    "subject_id" UUID NOT NULL,
    "class_id" UUID NOT NULL,
    "teacher_id" UUID NOT NULL,

    CONSTRAINT "subject_class_teacher_relation_pkey" PRIMARY KEY ("school_id","subject_id","class_id","teacher_id")
);

-- CreateTable
CREATE TABLE "subject_admin" (
    "school_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "subject_id" UUID NOT NULL,
    "permisson" VARBIT NOT NULL,

    CONSTRAINT "subject_admin_pkey" PRIMARY KEY ("school_id","user_id","subject_id")
);

-- CreateTable
CREATE TABLE "school_class" (
    "id" UUID NOT NULL,
    "school_id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "class_admin_id" UUID NOT NULL,
    "parent_class_id" UUID NOT NULL,
    "any_external_students" BOOLEAN NOT NULL,

    CONSTRAINT "school_class_pkey" PRIMARY KEY ("id","school_id")
);

-- CreateTable
CREATE TABLE "class_lesson_at_school_relation" (
    "lesson_id" UUID NOT NULL,
    "school_id" UUID NOT NULL,
    "class_id" UUID NOT NULL,

    CONSTRAINT "class_lesson_at_school_relation_pkey" PRIMARY KEY ("lesson_id","school_id","class_id")
);

-- CreateTable
CREATE TABLE "lesson" (
    "id" UUID NOT NULL,
    "school_id" UUID NOT NULL,
    "topic" TEXT NOT NULL,
    "attendance" JSONB NOT NULL,
    "starts" TIMESTAMP(3) NOT NULL,
    "ends" TIMESTAMP(3) NOT NULL,
    "canceled" BOOLEAN NOT NULL,
    "subject_id" UUID NOT NULL,
    "substitute_subject_id" UUID,
    "substitute_starts" TIMESTAMP(3),
    "substitute_ends" TIMESTAMP(3),

    CONSTRAINT "lesson_pkey" PRIMARY KEY ("id","school_id")
);

-- CreateTable
CREATE TABLE "subject" (
    "id" UUID NOT NULL,
    "school_id" UUID NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "subject_pkey" PRIMARY KEY ("id","school_id")
);

-- CreateTable
CREATE TABLE "mark_event" (
    "id" UUID NOT NULL,
    "school_id" UUID NOT NULL,
    "lesson_id" UUID,
    "class_id" UUID NOT NULL,
    "subject_id" UUID NOT NULL,
    "lesson_event_category_id" UUID NOT NULL,
    "associated_marks" BOOLEAN NOT NULL,
    "name" TEXT NOT NULL,
    "mark_type" "mark_type" NOT NULL,
    "value_type_value" INTEGER NOT NULL,
    "canceled" BOOLEAN NOT NULL,

    CONSTRAINT "mark_event_pkey" PRIMARY KEY ("id","school_id")
);

-- CreateTable
CREATE TABLE "mark" (
    "id" UUID NOT NULL,
    "school_id" UUID NOT NULL,
    "student_id" UUID NOT NULL,
    "lesson_event_id" UUID NOT NULL,
    "subject_id" UUID NOT NULL,
    "teacher_id" UUID NOT NULL,
    "description" TEXT NOT NULL,
    "value" INTEGER NOT NULL,

    CONSTRAINT "mark_pkey" PRIMARY KEY ("id","school_id")
);

-- CreateTable
CREATE TABLE "mark_event_category" (
    "id" UUID NOT NULL,
    "school_id" UUID NOT NULL,
    "allowed_mark_types_and_their_default_values" JSONB NOT NULL,
    "name" TEXT NOT NULL,
    "force_weigt_base" BOOLEAN NOT NULL,

    CONSTRAINT "mark_event_category_pkey" PRIMARY KEY ("id","school_id")
);


-- Distribution
CREATE EXTENSION citus;
SET LOCAL citus.multi_shard_modify_mode TO 'sequential';

SELECT create_reference_table('user');
SELECT create_reference_table('pii_data');
SELECT create_reference_table('session');

SELECT create_distributed_table('school',   'id');

SELECT create_distributed_table('subject',   'school_id');
SELECT create_distributed_table('lesson',   'school_id');
SELECT create_distributed_table('mark_event_category',   'school_id');
SELECT create_distributed_table('school_class',   'school_id');
SELECT create_distributed_table('mark_event',   'school_id');
SELECT create_distributed_table('class_lesson_at_school_relation',   'school_id');
SELECT create_distributed_table('school_settings',   'school_id');

SELECT create_distributed_table('mark',   'school_id');
SELECT create_distributed_table('subject_admin',   'school_id');
SELECT create_distributed_table('subject_class_teacher_relation',   'school_id');
SELECT create_distributed_table('user_class_admin_at_school_relation',   'school_id');
SELECT create_distributed_table('user_classes_at_school_relation',   'school_id');
SELECT create_distributed_table('user_lesson_external_student_at_school_relation',   'school_id');
SELECT create_distributed_table('user_lesson_teached_at_school_relation',   'school_id');
SELECT create_distributed_table('user_lesson_teached_substitute_at_school_relation',   'school_id');
SELECT create_distributed_table('user_school_relation',   'school_id');
-- CreateIndex
CREATE UNIQUE INDEX "session_session_id_key" ON "session"("session_id");

-- CreateIndex
CREATE INDEX "session_user_id_idx" ON "session"("user_id");

-- CreateIndex
CREATE INDEX "session_valid_until_idx" ON "session"("valid_until");

-- CreateIndex
CREATE UNIQUE INDEX "pii_data_user_id_key" ON "pii_data"("user_id");

-- CreateIndex
CREATE UNIQUE INDEX "pii_data_email_key" ON "pii_data"("email");

-- CreateIndex
CREATE UNIQUE INDEX "pii_data_phone_number_key" ON "pii_data"("phone_number");

-- CreateIndex
CREATE INDEX "user_school_relation_user_id_idx" ON "user_school_relation"("user_id");

-- CreateIndex
CREATE UNIQUE INDEX "school_settings_school_id_key" ON "school_settings"("school_id");

-- CreateIndex
CREATE INDEX "subject_class_teacher_relation_class_id_idx" ON "subject_class_teacher_relation"("class_id");

-- CreateIndex
CREATE INDEX "subject_admin_user_id_idx" ON "subject_admin"("user_id");

-- CreateIndex
CREATE INDEX "lesson_starts_ends_idx" ON "lesson"("starts", "ends");

-- AddForeignKey
ALTER TABLE "session" ADD CONSTRAINT "session_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pii_data" ADD CONSTRAINT "pii_data_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "user_school_relation" ADD CONSTRAINT "user_school_relation_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "user_school_relation" ADD CONSTRAINT "user_school_relation_school_id_fkey" FOREIGN KEY ("school_id") REFERENCES "school"("id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "user_classes_at_school_relation" ADD CONSTRAINT "user_classes_at_school_relation_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "user_classes_at_school_relation" ADD CONSTRAINT "user_classes_at_school_relation_school_id_fkey" FOREIGN KEY ("school_id") REFERENCES "school"("id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "user_classes_at_school_relation" ADD CONSTRAINT "user_classes_at_school_relation_class_id_school_id_fkey" FOREIGN KEY ("class_id", "school_id") REFERENCES "school_class"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "user_class_admin_at_school_relation" ADD CONSTRAINT "user_class_admin_at_school_relation_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "user_class_admin_at_school_relation" ADD CONSTRAINT "user_class_admin_at_school_relation_school_id_fkey" FOREIGN KEY ("school_id") REFERENCES "school"("id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "user_class_admin_at_school_relation" ADD CONSTRAINT "user_class_admin_at_school_relation_class_id_school_id_fkey" FOREIGN KEY ("class_id", "school_id") REFERENCES "school_class"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "user_lesson_teached_at_school_relation" ADD CONSTRAINT "user_lesson_teached_at_school_relation_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "user_lesson_teached_at_school_relation" ADD CONSTRAINT "user_lesson_teached_at_school_relation_school_id_fkey" FOREIGN KEY ("school_id") REFERENCES "school"("id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "user_lesson_teached_at_school_relation" ADD CONSTRAINT "user_lesson_teached_at_school_relation_lesson_id_school_id_fkey" FOREIGN KEY ("lesson_id", "school_id") REFERENCES "lesson"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "user_lesson_teached_substitute_at_school_relation" ADD CONSTRAINT "user_lesson_teached_substitute_at_school_relation_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "user_lesson_teached_substitute_at_school_relation" ADD CONSTRAINT "user_lesson_teached_substitute_at_school_relation_school_i_fkey" FOREIGN KEY ("school_id") REFERENCES "school"("id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "user_lesson_teached_substitute_at_school_relation" ADD CONSTRAINT "user_lesson_teached_substitute_at_school_relation_lesson_i_fkey" FOREIGN KEY ("lesson_id", "school_id") REFERENCES "lesson"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "user_lesson_external_student_at_school_relation" ADD CONSTRAINT "user_lesson_external_student_at_school_relation_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "user_lesson_external_student_at_school_relation" ADD CONSTRAINT "user_lesson_external_student_at_school_relation_school_id_fkey" FOREIGN KEY ("school_id") REFERENCES "school"("id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "user_lesson_external_student_at_school_relation" ADD CONSTRAINT "user_lesson_external_student_at_school_relation_lesson_id__fkey" FOREIGN KEY ("lesson_id", "school_id") REFERENCES "lesson"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "school_settings" ADD CONSTRAINT "school_settings_school_id_fkey" FOREIGN KEY ("school_id") REFERENCES "school"("id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "subject_class_teacher_relation" ADD CONSTRAINT "subject_class_teacher_relation_school_id_fkey" FOREIGN KEY ("school_id") REFERENCES "school"("id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "subject_class_teacher_relation" ADD CONSTRAINT "subject_class_teacher_relation_subject_id_school_id_fkey" FOREIGN KEY ("subject_id", "school_id") REFERENCES "subject"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "subject_class_teacher_relation" ADD CONSTRAINT "subject_class_teacher_relation_class_id_school_id_fkey" FOREIGN KEY ("class_id", "school_id") REFERENCES "school_class"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "subject_class_teacher_relation" ADD CONSTRAINT "subject_class_teacher_relation_teacher_id_fkey" FOREIGN KEY ("teacher_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "subject_admin" ADD CONSTRAINT "subject_admin_school_id_fkey" FOREIGN KEY ("school_id") REFERENCES "school"("id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "subject_admin" ADD CONSTRAINT "subject_admin_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "subject_admin" ADD CONSTRAINT "subject_admin_subject_id_school_id_fkey" FOREIGN KEY ("subject_id", "school_id") REFERENCES "subject"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "school_class" ADD CONSTRAINT "school_class_parent_class_id_school_id_fkey" FOREIGN KEY ("parent_class_id", "school_id") REFERENCES "school_class"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "school_class" ADD CONSTRAINT "school_class_school_id_fkey" FOREIGN KEY ("school_id") REFERENCES "school"("id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "class_lesson_at_school_relation" ADD CONSTRAINT "class_lesson_at_school_relation_lesson_id_school_id_fkey" FOREIGN KEY ("lesson_id", "school_id") REFERENCES "lesson"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "class_lesson_at_school_relation" ADD CONSTRAINT "class_lesson_at_school_relation_school_id_fkey" FOREIGN KEY ("school_id") REFERENCES "school"("id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "class_lesson_at_school_relation" ADD CONSTRAINT "class_lesson_at_school_relation_class_id_school_id_fkey" FOREIGN KEY ("class_id", "school_id") REFERENCES "school_class"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "lesson" ADD CONSTRAINT "lesson_subject_id_school_id_fkey" FOREIGN KEY ("subject_id", "school_id") REFERENCES "subject"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "lesson" ADD CONSTRAINT "lesson_substitute_subject_id_school_id_fkey" FOREIGN KEY ("substitute_subject_id", "school_id") REFERENCES "subject"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "subject" ADD CONSTRAINT "subject_school_id_fkey" FOREIGN KEY ("school_id") REFERENCES "school"("id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "mark_event" ADD CONSTRAINT "mark_event_lesson_id_school_id_fkey" FOREIGN KEY ("lesson_id", "school_id") REFERENCES "lesson"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "mark_event" ADD CONSTRAINT "mark_event_class_id_school_id_fkey" FOREIGN KEY ("class_id", "school_id") REFERENCES "school_class"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "mark_event" ADD CONSTRAINT "mark_event_subject_id_school_id_fkey" FOREIGN KEY ("subject_id", "school_id") REFERENCES "subject"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "mark_event" ADD CONSTRAINT "mark_event_lesson_event_category_id_school_id_fkey" FOREIGN KEY ("lesson_event_category_id", "school_id") REFERENCES "mark_event_category"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "mark" ADD CONSTRAINT "mark_student_id_fkey" FOREIGN KEY ("student_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "mark" ADD CONSTRAINT "mark_lesson_event_id_school_id_fkey" FOREIGN KEY ("lesson_event_id", "school_id") REFERENCES "mark_event"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "mark" ADD CONSTRAINT "mark_subject_id_school_id_fkey" FOREIGN KEY ("subject_id", "school_id") REFERENCES "subject"("id", "school_id") ON DELETE RESTRICT ON UPDATE RESTRICT;

-- AddForeignKey
ALTER TABLE "mark" ADD CONSTRAINT "mark_teacher_id_fkey" FOREIGN KEY ("teacher_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "mark_event_category" ADD CONSTRAINT "mark_event_category_school_id_fkey" FOREIGN KEY ("school_id") REFERENCES "school"("id") ON DELETE RESTRICT ON UPDATE RESTRICT;
