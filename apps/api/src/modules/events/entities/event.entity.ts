import { Device } from "src/modules/devices/entities/device.entity";
import { User } from "src/modules/users/entities/user.entity";
import { Column, Entity, Index, JoinColumn, ManyToOne, PrimaryGeneratedColumn } from "typeorm";

@Entity({ name: "events" })
export class Event {
  @PrimaryGeneratedColumn("uuid", { name: "event_id" })
  eventId: string;

  @Index()
  @Column({ type: "uuid", name: "user_id" })
  userId: string;

  @ManyToOne(() => User, (user) => user.events, { onDelete: "CASCADE" })
  @JoinColumn({ name: "user_id" })
  user: User;

  @Index()
  @Column({ type: "uuid", name: "device_id" })
  deviceId: string;

  @ManyToOne(() => Device, (device) => device.events, { onDelete: "SET NULL" })
  @JoinColumn({ name: "device_id" })
  device: Device;

  @Column({ type: "text" })
  content: string;

  @Column({ type: "text" })
  hash: string;

  @Column({ type: "timestamptz" })
  timestamp: Date;
}
