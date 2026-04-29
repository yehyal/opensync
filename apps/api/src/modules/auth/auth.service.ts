import { ConflictException, Injectable, UnauthorizedException } from "@nestjs/common";
import { JwtService } from "@nestjs/jwt";
import * as bcrypt from "bcryptjs";
import { createHash } from "crypto";
import { UsersService } from "../users/users.service";
import { AuthResponseDto } from "./dto/auth-response.dto";
import { LoginDto } from "./dto/login.dto";
import { RefreshDto } from "./dto/refresh.dto";
import { RegisterDto } from "./dto/register.dto";

@Injectable()
export class AuthService {
  constructor(
    private readonly usersService: UsersService,
    private readonly jwtService: JwtService,
  ) {}

  private hashRefreshToken(token: string): string {
    return createHash("sha256").update(token).digest("hex");
  }

  private async issueTokens(user: { id: string; email: string }): Promise<{
    accessToken: string;
    refreshToken: string;
  }> {
    const payload = { sub: user.id, email: user.email };

    const accessToken = await this.jwtService.signAsync(payload, {
      expiresIn: "15m",
    });

    const refreshToken = await this.jwtService.signAsync(payload, {
      expiresIn: "30d",
    });

    return { accessToken, refreshToken };
  }

  private toAuthResponse(
    user: any,
    tokens: { accessToken: string; refreshToken: string },
  ): AuthResponseDto {
    return {
      ...tokens,
      user: {
        id: user.id,
        email: user.email,
        displayName: user.displayName,
      },
    };
  }

  async register(dto: RegisterDto): Promise<AuthResponseDto> {
    const existing = await this.usersService.findByEmail(dto.email);
    if (existing) {
      throw new ConflictException("Email already in use");
    }

    const passwordHash = await bcrypt.hash(dto.password, 10);
    const user = await this.usersService.createWithPassword({
      email: dto.email,
      displayName: dto.displayName,
      passwordHash,
    });

    const tokens = await this.issueTokens(user);
    await this.usersService.setRefreshTokenHash(
      user.id,
      this.hashRefreshToken(tokens.refreshToken),
    );
    return this.toAuthResponse(user, tokens);
  }

  async login(dto: LoginDto): Promise<AuthResponseDto> {
    const user = await this.usersService.findByEmailWithPassword(dto.email);
    if (!user?.passwordHash) {
      throw new UnauthorizedException("Invalid credentials");
    }

    const ok = await bcrypt.compare(dto.password, user.passwordHash);
    if (!ok) {
      throw new UnauthorizedException("Invalid credentials");
    }

    const tokens = await this.issueTokens(user);
    await this.usersService.setRefreshTokenHash(
      user.id,
      this.hashRefreshToken(tokens.refreshToken),
    );
    return this.toAuthResponse(user, tokens);
  }

  async refresh(dto: RefreshDto): Promise<AuthResponseDto> {
    let payload: any;
    try {
      payload = await this.jwtService.verifyAsync(dto.refreshToken, {
        secret: process.env.JWT_REFRESH_SECRET ?? "dev_refresh_secret",
      });
    } catch {
      throw new UnauthorizedException("Invalid refresh token");
    }

    const user = await this.usersService.findByEmailWithPassword(payload.email);
    if (!user) {
      throw new UnauthorizedException("Invalid refresh token");
    }

    const expectedHash = user.refreshTokenHash;
    const actualHash = this.hashRefreshToken(dto.refreshToken);
    if (!expectedHash || expectedHash !== actualHash) {
      throw new UnauthorizedException("Invalid refresh token");
    }

    const tokens = await this.issueTokens(user);
    await this.usersService.setRefreshTokenHash(
      user.id,
      this.hashRefreshToken(tokens.refreshToken),
    );
    return this.toAuthResponse(user, tokens);
  }

  async logout(userId: string): Promise<void> {
    await this.usersService.setRefreshTokenHash(userId, null);
  }
}
