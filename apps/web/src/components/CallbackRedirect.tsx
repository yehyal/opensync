import { useEffect } from "react";

type CallbackRedirectProps = {
  nextUrl: string;
};

export function CallbackRedirect({ nextUrl }: CallbackRedirectProps) {
  useEffect(() => {
    window.location.replace(nextUrl);
  }, [nextUrl]);

  return (
    <a
      className="button button--primary auth-callback-action"
      href={nextUrl}
    >
      Open the app
    </a>
  );
}

export default CallbackRedirect;
