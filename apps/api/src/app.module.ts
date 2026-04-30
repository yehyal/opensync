import { Module } from "@nestjs/common";
import { TypeOrmModule } from "@nestjs/typeorm";
import { AppController } from "./app.controller";
import { AppService } from "./app.service";
import { typeOrmConfig } from "./db/typeorm.config";
import { EventsModule } from "./modules/events/events.module";
import { AuthModule } from "./modules/auth/auth.module";
import { UsersModule } from "./modules/users/users.module";
import { DevicesModule } from "./modules/devices/devices.module";

@Module({
  imports: [
    TypeOrmModule.forRoot(typeOrmConfig),
    EventsModule,
    UsersModule,
    AuthModule,
    DevicesModule,
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
