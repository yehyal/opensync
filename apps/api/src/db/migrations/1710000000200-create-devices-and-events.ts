import { MigrationInterface, QueryRunner } from "typeorm";

export class CreateDevicesAndEvents1710000000200 implements MigrationInterface {
  name = "CreateDevicesAndEvents1710000000200";

  public async up(queryRunner: QueryRunner): Promise<void> {
    await queryRunner.query(`
      CREATE TABLE "devices" (
        "device_id" uuid NOT NULL DEFAULT uuid_generate_v4(),
        "user_id" uuid NOT NULL,
        "device_name" text NOT NULL,
        "last_seen" timestamptz,
        "platform" text NOT NULL,
        CONSTRAINT "PK_devices_device_id" PRIMARY KEY ("device_id"),
        CONSTRAINT "FK_devices_user" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE
      );
    `);

    await queryRunner.query(`CREATE INDEX "IDX_devices_user_id" ON "devices" ("user_id");`);

    await queryRunner.query(`
      CREATE TABLE "events" (
        "event_id" uuid NOT NULL DEFAULT uuid_generate_v4(),
        "user_id" uuid NOT NULL,
        "device_id" uuid NOT NULL,
        "content" text NOT NULL,
        "hash" text NOT NULL,
        "timestamp" timestamptz NOT NULL DEFAULT now(),
        CONSTRAINT "PK_events_event_id" PRIMARY KEY ("event_id"),
        CONSTRAINT "FK_events_user" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE,
        CONSTRAINT "FK_events_device" FOREIGN KEY ("device_id") REFERENCES "devices"("device_id") ON DELETE CASCADE
      );
    `);

    await queryRunner.query(`CREATE INDEX "IDX_events_user_id" ON "events" ("user_id");`);
    await queryRunner.query(`CREATE INDEX "IDX_events_device_id" ON "events" ("device_id");`);
    await queryRunner.query(`CREATE INDEX "IDX_events_hash" ON "events" ("hash");`);
  }

  public async down(queryRunner: QueryRunner): Promise<void> {
    await queryRunner.query(`DROP INDEX IF EXISTS "IDX_events_hash";`);
    await queryRunner.query(`DROP INDEX IF EXISTS "IDX_events_device_id";`);
    await queryRunner.query(`DROP INDEX IF EXISTS "IDX_events_user_id";`);
    await queryRunner.query(`DROP TABLE "events";`);

    await queryRunner.query(`DROP INDEX IF EXISTS "IDX_devices_user_id";`);
    await queryRunner.query(`DROP TABLE "devices";`);
  }
}
