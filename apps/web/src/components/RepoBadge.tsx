import { useEffect, useState } from "react";

type RepoBadgeProps = {
  repo: string;
  href: string;
  showName?: boolean;
  className?: string;
};

function GitHubIcon() {
  return (
    <svg aria-hidden="true" viewBox="0 0 24 24" className="repo-badge__icon">
      <path
        fill="currentColor"
        d="M12 .5a12 12 0 0 0-3.79 23.39c.6.11.82-.26.82-.58v-2.23c-3.34.73-4.04-1.41-4.04-1.41-.55-1.39-1.33-1.76-1.33-1.76-1.09-.75.08-.74.08-.74 1.2.09 1.84 1.24 1.84 1.24 1.08 1.84 2.82 1.31 3.5 1 .1-.79.42-1.31.77-1.61-2.67-.31-5.47-1.33-5.47-5.93 0-1.31.47-2.37 1.24-3.21-.13-.31-.54-1.57.12-3.28 0 0 1.01-.32 3.31 1.23a11.5 11.5 0 0 1 6.03 0c2.3-1.55 3.3-1.23 3.3-1.23.67 1.71.26 2.97.13 3.28.77.84 1.23 1.9 1.23 3.21 0 4.61-2.81 5.61-5.49 5.91.43.37.82 1.1.82 2.23v3.3c0 .32.22.7.83.58A12 12 0 0 0 12 .5Z"
      />
    </svg>
  );
}

function StarIcon() {
  return (
    <svg aria-hidden="true" viewBox="0 0 24 24" className="repo-badge__icon repo-badge__icon--star">
      <path
        fill="currentColor"
        d="m12 2.75 2.87 5.82 6.43.93-4.65 4.53 1.1 6.4L12 17.41 6.25 20.43l1.1-6.4L2.7 9.5l6.43-.93L12 2.75Z"
      />
    </svg>
  );
}

function formatStars(value: number) {
  return new Intl.NumberFormat("en", {
    notation: "compact",
    maximumFractionDigits: value >= 1000 ? 1 : 0,
  }).format(value);
}

export function RepoBadge({ repo, href, showName = false, className = "" }: RepoBadgeProps) {
  const [stars, setStars] = useState<number | null>(null);

  useEffect(() => {
    let active = true;

    async function loadRepo() {
      try {
        const response = await fetch(`https://api.github.com/repos/${repo}`, {
          headers: {
            accept: "application/vnd.github+json",
          },
        });

        if (!response.ok) {
          return;
        }

        const data = (await response.json()) as { stargazers_count?: number };

        if (active && typeof data.stargazers_count === "number") {
          setStars(data.stargazers_count);
        }
      } catch {
        if (active) {
          setStars(null);
        }
      }
    }

    void loadRepo();

    return () => {
      active = false;
    };
  }, [repo]);

  return (
    <a
      className={`repo-badge ${className}`.trim()}
      href={href}
      target="_blank"
      rel="noreferrer"
      aria-label={
        stars === null ? `Open ${repo} on GitHub` : `Open ${repo} on GitHub, ${stars} stars`
      }
    >
      <GitHubIcon />
      {showName ? <span className="repo-badge__name">{repo}</span> : null}
      <span className="repo-badge__stars">
        <StarIcon />
        <span>{stars === null ? "GitHub" : formatStars(stars)}</span>
      </span>
    </a>
  );
}

export default RepoBadge;
