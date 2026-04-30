import { Module } from "@nestjs/common";
import { TypeOrmModule } from "@nestjs/typeorm";
import { User } from "./entities/user.entity";
import { UsersController } from "./users.controller";
import { UsersService } from "./users.service";
import { DevicesModule } from "../devices/devices.module";
import { EventsModule } from "../events/events.module";

@Module({
  imports: [TypeOrmModule.forFeature([User]), DevicesModule, EventsModule],
  controllers: [UsersController],
  providers: [UsersService],
  exports: [UsersService],
})
export class UsersModule {}
