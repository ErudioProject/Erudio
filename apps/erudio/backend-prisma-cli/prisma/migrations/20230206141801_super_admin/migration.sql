-- CreateTable
CREATE TABLE "super_admin" (
    "id" UUID NOT NULL,
    "password_hash" TEXT NOT NULL,
    "login" TEXT NOT NULL,

    CONSTRAINT "super_admin_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "super_admin_login_key" ON "super_admin"("login");
