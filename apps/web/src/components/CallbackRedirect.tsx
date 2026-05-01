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
      className="inline-flex items-center justify-center bg-[#1f5eff] text-white px-5 py-3 no-underline"
      href={nextUrl}
    >
      Open the app
    </a>
  );
}

export default CallbackRedirect;
