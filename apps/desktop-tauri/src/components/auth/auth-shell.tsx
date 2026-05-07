import type { PropsWithChildren } from "react";
import { Card, CardContent } from "../ui/card";

export function AuthShell({ children }: PropsWithChildren) {
  return (
    <main className="flex min-h-screen overflow-hidden overscroll-none">
      <Card className="flex-1 max-w-lg overflow-hidden rounded-none">
        <CardContent className="p-0">
          <div className="px-8 py-10">{children}</div>
        </CardContent>
      </Card>
    </main>
  );
}
