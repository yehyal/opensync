---
name: OpenSync
description: Calm sync across devices, with a warm brand voice and crisp technical actions.
colors:
  primary: "#1f5eff"
  primary-soft: "#e6eeff"
  neutral-bg: "#f8f4ee"
  neutral-bg-alt: "#f3efe7"
  neutral-surface: "#fffdf8"
  neutral-border: "#d9d0c0"
  neutral-ink: "#1c1a17"
  neutral-muted: "#6c665d"
  desktop-bg: "#09090b"
  desktop-bg-alt: "#111827"
  desktop-accent: "#4ade80"
  desktop-accent-soft: "#86efac"
  desktop-ink: "#f5efe5"
typography:
  display:
    fontFamily: "Iowan Old Style, Palatino Linotype, Book Antiqua, Palatino, Georgia, serif"
    fontSize: "clamp(2rem, 6vw, 3.2rem)"
    fontWeight: 600
    lineHeight: 0.95
  body:
    fontFamily: "Iowan Old Style, Palatino Linotype, Book Antiqua, Palatino, Georgia, serif"
    fontSize: "1rem"
    fontWeight: 400
    lineHeight: 1.6
  label:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "0.72rem"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "0.18em"
rounded:
  sm: "8px"
  md: "16px"
  lg: "24px"
spacing:
  sm: "12px"
  md: "16px"
  lg: "20px"
  xl: "32px"
  xxl: "40px"
components:
  button-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.neutral-surface}"
    rounded: "{rounded.sm}"
    padding: "12px 20px"
  button-secondary:
    backgroundColor: "{colors.desktop-accent}"
    textColor: "{colors.desktop-bg}"
    rounded: "{rounded.md}"
    padding: "12px 24px"
  panel-web:
    backgroundColor: "{colors.neutral-surface}"
    textColor: "{colors.neutral-ink}"
    rounded: "{rounded.md}"
    padding: "32px"
  panel-desktop:
    backgroundColor: "{colors.desktop-bg-alt}"
    textColor: "{colors.desktop-ink}"
    rounded: "{rounded.lg}"
    padding: "32px"
---

# Design System: OpenSync

## Overview

**Creative North Star: "Quiet"**

OpenSync should feel like access without ceremony. The visual system is soft on contact, cream-toned, breathable, and calm at first glance, then sharply decisive at the moment of action. The current web implementation is the primary voice: warm paper-like neutrals, a single vivid cobalt action color, serif-forward display typography, and generous spacing that makes a technical product feel unhurried.

The desktop client is a secondary expression of the same brand, not a separate identity. It can run darker and more utility-driven when the environment calls for it, but it should still inherit the same emotional contract: quiet surfaces, low-friction flows, and crisp, confident interaction points. The brand rejects cloud-suite heaviness, noisy management UI, and any presentation that makes a simple sync story feel complicated.

**Key Characteristics:**
- Soft surfaces with crisp actions.
- Warm, lightly editorial calm on the web.
- Technically fluent language without visual aggression.
- Low-density layouts that preserve breathing room around the core action.
- Clear continuity between marketing and product surfaces.

## Colors

The palette is web-first and restrained: warm neutrals carry most of the surface, while one saturated action color does the speaking.

### Primary
- **Signal Cobalt** (`#1f5eff`): the single decisive action color. Use it for primary buttons, active links, and high-intent states where the user commits to the next step.

### Secondary
- **Relay Green** (`#4ade80`): a secondary accent reserved for the desktop client and authenticated success states. It signals connection, completion, and live system feedback.

### Neutral
- **Paper Glow** (`#f8f4ee`): the main web background, used for full-page fields that need warmth without visual noise.
- **Warm Mist** (`#f3efe7`): the lower step in the web background gradient. Use it to keep large surfaces from feeling flat.
- **Quiet Ivory** (`#fffdf8`): the primary panel and card surface for auth screens and focused content blocks.
- **Soft Divider** (`#d9d0c0`): borders and input outlines. It should separate, never shout.
- **Inkstone** (`#1c1a17`): primary web copy. Use for headlines and body text that needs full authority.
- **Muted Clay** (`#6c665d`): supporting copy, labels, and secondary links.
- **Night Field** (`#09090b`): the deepest desktop background.
- **Slate Depth** (`#111827`): desktop panel and gradient support color.
- **Shell Light** (`#f5efe5`): primary desktop text on dark surfaces.

**The One Voice Rule.** The primary accent is not a decorative wash. Signal Cobalt is used sparingly and deliberately. If a screen starts feeling blue, it has already lost restraint.

## Typography

**Display Font:** Iowan Old Style, Palatino Linotype, Book Antiqua, Palatino, Georgia, serif
**Body Font:** Iowan Old Style, Palatino Linotype, Book Antiqua, Palatino, Georgia, serif
**Label Font:** Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif

**Character:** The web system uses a literary serif texture to make a technical product feel calm, human, and unforced. Sans labels appear only where precision matters, as interface metadata rather than as the page's dominant voice.

### Hierarchy
- **Display** (`600`, `clamp(2rem, 6vw, 3.2rem)`, `0.95`): hero headlines and auth-screen titles. Tight leading keeps the message compact and certain.
- **Headline** (`600`, `2rem`, `1.05`): secondary hero moments and desktop app titles that need weight without theatricality.
- **Title** (`600`, `1.25rem`, `1.2`): section titles, status headings, and primary component headings.
- **Body** (`400`, `1rem`, `1.6`): primary explanatory text. Cap line length at roughly `65ch` to preserve calm reading rhythm.
- **Label** (`600`, `0.72rem`, `0.18em`): short metadata only, including product kickers and small auth labels. Keep it brief and intentional.

**The Whispered Label Rule.** Labels can guide the eye, but they must never become a repeated stylistic tic. Use them for orientation, not as decoration above every block.

## Elevation

OpenSync uses ambient elevation rather than hard UI stacking. Depth comes from soft panels, subtle border definition, and wide blurred shadows that feel atmospheric instead of mechanical. The web side uses shadow as gentle separation over warm gradients; the desktop side uses deeper shadows to keep dark panels readable without looking glossy.

### Shadow Vocabulary
- **Warm Lift** (`0 24px 64px rgba(22, 18, 9, 0.08)`): the default web panel shadow for centered auth surfaces and focused content blocks.
- **Desktop Well** (`0 24px 80px rgba(15, 23, 42, 0.45)`): the desktop shell shadow for dark elevated panels.
- **Action Glow** (`0 14px 40px rgba(74, 222, 128, 0.25)`): the desktop primary-action halo. Use only on direct action controls.

**The Atmosphere Rule.** Shadows should read like air and distance, not like plastic. If the edge looks hard or the effect feels shiny, the shadow is wrong.

## Components

Each component should feel easy to approach and exact to use. Corners are softened, spacing is generous, and actions land with more contrast than containers.

### Buttons
- **Shape:** gently squared with softened corners (`8px` on web primary buttons, `12px to 16px` in the desktop client).
- **Primary:** use Signal Cobalt (`#1f5eff`) on the web with Quiet Ivory text (`#fffdf8`) and `12px 20px` padding. In the desktop client, use Relay Green (`#4ade80`) with dark text (`#09090b`) and taller sizing (`48px` height, `24px` horizontal padding).
- **Hover / Focus:** hover should slightly brighten the active color. Focus uses a visible ring in the accent family, especially on desktop where `ring-emerald-300/70` is already the pattern.
- **Secondary / Ghost:** the desktop secondary button is a translucent white surface (`bg-white/8`) with a faint border and restrained hover lift. Secondary actions should feel available, never equal to the primary.

### Cards / Containers
- **Corner Style:** web panels use medium rounding (`16px` implied by the visual treatment), while desktop panels can stretch to larger shells (`24px`) when they need more atmosphere.
- **Background:** web panels use Quiet Ivory (`#fffdf8`) at high opacity over warm gradients. Desktop panels use translucent light-on-dark surfaces over Slate Depth.
- **Shadow Strategy:** use Warm Lift on web panels and Desktop Well on desktop shells. Do not stack shadows.
- **Border:** always quiet, usually Soft Divider (`#d9d0c0`) on web or low-opacity white on desktop.
- **Internal Padding:** default to `32px` with room to expand to `40px` on larger auth blocks.

### Inputs / Fields
- **Style:** web inputs are plain, rectangular, and calm: white background, Soft Divider border (`#d9d0c0`), and dark ink text (`#1c1a17`) with `16px 12px` inner padding.
- **Focus:** focus should sharpen the field through border contrast or a restrained accent ring, not through heavy glow.
- **Error / Disabled:** preserve the same quiet geometry. State changes should come from color and message clarity, not from changing shape.

### Navigation
- **Style:** links are understated by default, often Muted Clay (`#6c665d`) with underline treatments rather than boxed controls. Primary navigation actions can escalate to full button treatment when the decision matters.
- **State:** active or high-intent paths switch to the primary accent. Hover should feel crisp, not animated for its own sake.
- **Mobile Treatment:** preserve the same calm spacing and warm surfaces. Avoid compressed, dashboard-like top bars on small screens.

### Status and Authentication Surfaces
- **Style:** waiting, authenticated, and sign-in states should stay visually sparse. One status icon or spinner, one clear heading, one compact explanatory line.
- **State:** success can borrow Relay Green, but only for the immediate confirmation moment.
- **Behavior:** the authentication flow should feel automatic and self-updating, not like a manual configuration task.

## Do's and Don'ts

### Do:
- **Do** lead with warm neutrals on the web: Paper Glow (`#f8f4ee`), Warm Mist (`#f3efe7`), and Quiet Ivory (`#fffdf8`) should carry most of the page.
- **Do** make the main action unmistakable with Signal Cobalt (`#1f5eff`) or Relay Green (`#4ade80`), then let everything else stay quieter.
- **Do** keep layouts breathable, especially around login, signup, and first-contact marketing moments.
- **Do** preserve the same calm mental model when moving from landing page to desktop client, even if the desktop environment runs darker.
- **Do** use motion purposefully. Small spinners, fades, and state transitions are welcome when they clarify system status.

### Don't:
- **Don't** make the interface feel overwhelming, crowded, or operationally heavy.
- **Don't** drift toward visual or interaction patterns of complex cloud suites that foreground management, configuration, or feature sprawl.
- **Don't** use noisy dashboard styling, dense enterprise framing, or marketing language that makes a simple idea feel complicated.
- **Don't** let the desktop client become a separate cyber-tool aesthetic with unrelated colors, unrelated type, or gratuitous dark-mode theatrics.
- **Don't** flood the page with the accent color, stack shadows, or replace calm surfaces with flashy decorative effects.
