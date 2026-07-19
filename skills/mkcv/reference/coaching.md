# Coaching playbook

Consult this during **Phase 3** (interview) and **Phase 5** (refine) of the
workflow in `SKILL.md`. It exists because the hard part of a resume isn't
formatting — it's getting strong, specific, job-relevant content out of a person
who often undersells themselves. Use it to decide what to ask and how to rewrite.

## Contents
1. The bullet formula
2. Interview question bank (surface impact, don't just fill fields)
3. Job-alignment tactics
4. Common weaknesses and their fixes
5. Template selection guidance
6. Length and honesty

---

## 1. The bullet formula

A strong bullet is **action verb → what you did → quantified impact**. The impact
is what most people omit, and it's what hiring managers actually read for.

- Weak: "Responsible for the payments database."
- Strong: "Cut p99 query latency 40% by redesigning the indexing strategy for a
  2 TB Postgres cluster serving 12M daily transactions."

Levers to pull when a bullet is flat:
- **Add a number.** Percent change, scale (users/req-per-sec/dollars/rows), team
  size, time saved. If the user doesn't have the exact figure, ask for a
  defensible estimate ("roughly how much faster?") rather than leaving it vague.
- **Lead with a strong verb.** Led, shipped, cut, scaled, designed, automated,
  reduced, drove — not "responsible for", "helped with", "worked on".
- **Name the stakes.** Why did it matter? Revenue, reliability, a launch, a
  deadline, unblocking a team.
- **Keep one idea per bullet.** Split compound bullets.

When the user describes something only vaguely, *generate* a candidate bullet and
ask them to confirm/correct — a concrete draft is easier to react to than a blank
"tell me more". Mark any invented specifics with `‹?›` until confirmed.

---

## 2. Interview question bank

Ask the questions that close the gaps between a bland draft and a strong one.
Pick what's relevant to what's missing; don't run the whole list.

**Impact & scale (highest leverage — most resumes are weakest here):**
- "What changed *because* of your work? Any metric that moved?"
- "How big was it — users, requests, revenue, data size, team?"
- "What would have gone wrong if you hadn't done it?"
- "What are you most proud of shipping in this role?"

**Seniority & scope:**
- "Did you lead people or projects? How many, how long?"
- "What did you own end-to-end vs. contribute to?"
- "Any cross-team or cross-org work, or influence beyond your team?"

**Fit for the target (see §3):**
- "This role emphasizes X — where have you done that?"
- "What of your experience is *most* relevant to the job you want?"

**Gap-filling:**
- Dates/titles for each role; education; certifications; notable projects,
  open-source, talks, or publications (the schema has `projects`/`honors`, and
  academic CVs may want a custom section).
- Contact/links (email, GitHub, LinkedIn, portfolio).

Stop interviewing when you could state in two sentences *why this person is a
strong candidate for their target* — coherent story beats filled fields.

---

## 3. Job-alignment tactics

A resume tailored to a specific job beats a generic one, so this is the single
biggest quality lever. Apply it whenever the user gives a target role or posting.

1. **Mine the posting.** Extract its must-haves, priorities, and the exact
   language it uses (tools, methodologies, outcomes). If given a URL, `WebFetch`
   it; if a paste, read it directly.
2. **Reorder for relevance.** Put the experience and bullets that match the
   posting's top priorities first — recruiters skim top-down. Use `meta.order` or
   bullet order to surface the most relevant evidence.
3. **Mirror real keywords.** Where the user *genuinely* has the experience, use
   the posting's own terms (many resumes are first read by keyword filters).
   **Never fabricate** a skill to match — align, don't lie.
4. **Rephrase toward their outcomes.** If the posting cares about reliability,
   frame relevant work in terms of uptime/latency/incident reduction.
5. **Tell the user what you changed and why** — "I moved your Kubernetes work to
   the top and reworded it to match their 'platform reliability' language."

If there's no target job, write a strong general resume but note that giving you
a target is the most impactful next step.

---

## 4. Common weaknesses → fixes

- **Duty lists, no outcomes** → apply the bullet formula (§1); ask for numbers.
- **Vague seniority** → probe ownership and leadership (§2).
- **Kitchen-sink skills** → group into a small taxonomy (languages / frameworks /
  infra / tools); cut the noise; put target-relevant skills first.
- **Wall-of-text summary** → 2–3 lines: who they are, their strongest evidence,
  what they're targeting.
- **Inconsistent tense/person** → past tense for past roles, present for current;
  no "I".
- **Overflowing to 2+ pages early-career** → tighten to one (see §6).

---

## 5. Template selection guidance

- `modern` — minimalist, safe, ATS-friendly default. Good for most tech/business
  roles.
- `crisp` — polished, a bit more designed (two-tone name, accented sections).
  Strong for experienced ICs who want to stand out modestly.
- `serif` — traditional single-column; the most conservative / ATS-safe. Good for
  formal industries or when applying through automated systems.
- `split` — two-column, compact; fits a lot on one page.
- `formal` (CV) — classic CV with left-margin dates; academic/formal.
- `sidebar` (CV) — colored sidebar, skill tags, supports a **photo**; the most
  visually striking. Good when standing out helps and ATS isn't the gate.

Match to the user's field and how conservative the target is. When unsure, render
their draft in 2–3 and let them pick visually (Phase 2).

---

## 6. Length and honesty

- One page for early/mid career; two only when the history genuinely warrants it.
  Academic CVs can run longer (publications, talks).
- Everything on the resume must be true. You may *phrase* generously and *infer*
  structure, but invented facts stay marked `‹?›` until the user confirms them,
  and unconfirmed guesses never survive to the final PDF.
