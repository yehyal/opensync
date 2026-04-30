import { Event } from "src/modules/events/entities/event.entity";
import { User } from "src/modules/users/entities/user.entity";
import {
  Column,
  Entity,
  Index,
  JoinColumn,
  ManyToOne,
  OneToMany,
  PrimaryGeneratedColumn,
} from "typeorm";

@Entity({ name: "devices" })
export class Device {
  @PrimaryGeneratedColumn("uuid", { name: "device_id" })
  deviceId: string;

  @Index()
  @Column({ type: "uuid", name: "user_id" })
  userId: string;

  @ManyToOne(() => User, (user) => user.devices, { onDelete: "CASCADE" })
  @JoinColumn({ name: "user_id" })
  user: User;

  @Column({ type: "text", name: "device_name" })
  deviceName: string;

  @Column({ type: "timestamptz", name: "last_seen", nullable: true })
  lastSeen: Date | null;

  @Column({ type: "text" })
  platform: string;

  @OneToMany(() => Event, (event) => event.device)
  events: Event[];
}
