#!/usr/bin/env python3
"""Split plan 21 §7 (complete TwinCAT API catalog) into per-library
documents under docs/plc/compat/twincat/.

Each `### <product> — `<manual>`` section of the plan becomes one
markdown file; an INDEX.md table links them all. Regenerate after
updating docs/plc/plan/21-twincat-libraries.md.
"""
import re, sys
from pathlib import Path

PLAN = Path('/home/uli/dev/TetherPLC/docs/plc/plan/21-twincat-libraries.md')
OUT = Path('/home/uli/dev/TetherPLC/docs/plc/compat/twincat')

text = PLAN.read_text()
# isolate section 7 (catalog) — it runs to the '## 8.' roadmap heading
m = re.search(r'^## 7\. Complete recursive API catalog\n(.*?)^## 8\.',
              text, re.S | re.M)
if not m:
    sys.exit('catalog section not found')
cat = m.group(1)

# split on '### <product> — `<manual>`' headers
parts = re.split(r'(?m)^(### .+?)$', cat)
# parts[0] = preamble; then alternating header/body
libs = []
for i in range(1, len(parts), 2):
    head, body = parts[i], parts[i + 1] if i + 1 < len(parts) else ''
    hm = re.match(r'### (.+?) — `([^`]+)`', head)
    if not hm:
        continue
    product, manual = hm.group(1), hm.group(2)
    libs.append((product, manual, body.strip()))

def slug(s):
    return re.sub(r'[^a-z0-9]+', '-', s.lower()).strip('-')

OUT.mkdir(parents=True, exist_ok=True)
# clear stale generated files (keep INDEX.md handled below)
for f in OUT.glob('*.md'):
    f.unlink()

index = ['# TwinCAT compatibility library docs', '',
         'Per-library extracts of the complete API catalog in',
         '`docs/plc/plan/21-twincat-libraries.md` §7. Each file lists',
         'every documented function, function block, method, property,',
         'and event with parameters and descriptions, plus the',
         'TetherPLC integration tier and requirements.', '',
         '| # | Library | Manual | APIs |', '|---|---|---|---|']

for n, (product, manual, body) in enumerate(libs, 1):
    meta = {}
    for line in body.split('\n')[:6]:
        if not line.startswith('- **'):
            break
        for k, v in re.findall(
                r'\*\*([^:*]+):\*\*\s*(.*?)(?=\s*·\s*\*\*[^:*]+:\*\*|$)',
                line):
            meta[k.strip()] = v.strip()
    tier = meta.get('Tier', '?').split(' ')[0]
    catg = meta.get('Category', '?')
    apis = meta.get('API entries', '?')
    note = meta.get('Integration', '')
    title = f'{n:02d}-{slug(product)}.md'
    doc = [f'# {product}', '',
           f'- **Manual:** `{manual}` (Beckhoff TwinCAT 3 PLC library',
           '  documentation; copyright Beckhoff Automation)',
           f'- **Category:** {catg} · **Tier:** {tier} · '
           f'**API entries:** {apis}',
           f'- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`',
           '  §7 (complete recursive catalog)', '',
           f'**Integration:** {note}', '',
           '---', '']
    # body starts with the '- **Category:**' meta lines then #### sections;
    # drop the meta bullets (already in header) but keep #### content
    content = re.sub(r'\A(?:- \*\*[^\n]*\n)+\n?', '', body)
    doc.append(content.rstrip() + '\n')
    (OUT / title).write_text('\n'.join(doc))
    index.append(f'| {n} | [{product}]({title}) '
                 f'| `{manual}` | {apis} |')

(OUT / 'INDEX.md').write_text('\n'.join(index) + '\n')
print(f'{len(libs)} library docs -> {OUT}')
