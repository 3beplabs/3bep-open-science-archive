import urllib.request
import urllib.parse
import xml.etree.ElementTree as ET
import json
import os
import sys

# Search Configuration
CATEGORIES = ["astro-ph.EP", "astro-ph.GA", "nlin.CD", "physics.comp-ph"]
# Searching for papers mentioning n-body simulations AND integration/stability issues
DOMAIN_KEYWORDS = ["n-body", "three-body", "gravitational", "orbital dynamics", "simulation"]
FRAGILITY_KEYWORDS = ["energy drift", "numerical instability", "integration error", "symplectic", "floating point", "round-off", "lyapunov exponent", "chaotic divergence", "numerical artifact"]

HISTORY_FILE = os.path.join(os.path.dirname(__file__), "history.json")

def load_history():
    if os.path.exists(HISTORY_FILE):
        with open(HISTORY_FILE, "r", encoding="utf-8") as f:
            return json.load(f)
    return {"processed_ids": []}

def save_history(history):
    with open(HISTORY_FILE, "w", encoding="utf-8") as f:
        json.dump(history, f, indent=2)

def fetch_arxiv_papers(max_results=50):
    cat_query = "+OR+".join([f"cat:{c}" for c in CATEGORIES])
    query = f"search_query={cat_query}&sortBy=submittedDate&sortOrder=descending&max_results={max_results}"
    url = f"https://export.arxiv.org/api/query?{query}"
    
    print(f"[*] Querying arXiv API: {url}")
    try:
        response = urllib.request.urlopen(url)
        xml_data = response.read()
        return ET.fromstring(xml_data)
    except Exception as e:
        print(f"[!] Error querying arXiv: {e}")
        sys.exit(1)

import tarfile
import io
import time

CODE_EXTENSIONS = {'.c', '.cpp', '.f90', '.f', '.f95', '.py', '.jl', '.m', '.rs', '.h', '.hpp'}

def check_source_code(paper_id):
    eprint_url = f"https://export.arxiv.org/e-print/{paper_id}"
    found_code_files = []
    
    print(f"    [>] Checking source code attachments for {paper_id}...")
    time.sleep(3) # Politeness required by arXiv API (anti Error 429)
    try:
        # Request with Timeout to avoid blocking the bot if arXiv is slow
        req = urllib.request.Request(eprint_url, headers={'User-Agent': '3bep-audit-bot/1.0'})
        response = urllib.request.urlopen(req, timeout=10)
        
        # arXiv e-print might return the PDF directly if there is no source bundle, 
        # but usually returns a TAR GZ with the .tex and attachments.
        if response.info().get_content_type() == 'application/x-eprint-tar':
            tar_bytes = response.read()
            with tarfile.open(fileobj=io.BytesIO(tar_bytes), mode="r:gz") as tar:
                for member in tar.getmembers():
                    if member.isfile():
                        ext = os.path.splitext(member.name)[1].lower()
                        if ext in CODE_EXTENSIONS:
                            found_code_files.append(member.name)
                            
    except Exception as e:
        print(f"    [!] Warning downloading e-print for {paper_id}: {e}")
        pass
        
    return found_code_files

def analyze_papers(root, history):
    candidates = []
    ns = {'atom': 'http://www.w3.org/2005/Atom'}
    
    for entry in root.findall('atom:entry', ns):
        # Handles arxiv versions (e.g., 2404.12345v1 -> 2404.12345)
        raw_id = entry.find('atom:id', ns).text.split('/abs/')[-1]
        paper_id = raw_id.split('v')[0] if 'v' in raw_id else raw_id
        
        if paper_id in history["processed_ids"]:
            continue
            
        title = entry.find('atom:title', ns).text.replace('\n', ' ').strip()
        summary = entry.find('atom:summary', ns).text.replace('\n', ' ').strip()
        authors = [a.find('atom:name', ns).text for a in entry.findall('atom:author', ns)]
        link = entry.find("atom:link[@title='pdf']", ns)
        pdf_url = link.attrib['href'] if link is not None else entry.find('atom:id', ns).text
        
        summary_lower = summary.lower()
        title_lower = title.lower()
        content_to_check = summary_lower + " " + title_lower
        
        has_domain = any(kw in content_to_check for kw in DOMAIN_KEYWORDS)
        found_fragilities = [kw for kw in FRAGILITY_KEYWORDS if kw in content_to_check]
        
        if has_domain and found_fragilities:
            kw = found_fragilities[0]
            idx = summary_lower.find(kw)
            start_idx = max(0, idx - 80)
            end_idx = min(len(summary), idx + len(kw) + 80)
            snippet = summary[start_idx:end_idx].strip()
            if start_idx > 0: snippet = "..." + snippet
            if end_idx < len(summary): snippet = snippet + "..."
            
            # Feature: Download e-print info to scan for source codes
            code_files = check_source_code(paper_id)
            
            candidates.append({
                "id": paper_id,
                "title": title,
                "authors": ", ".join(authors),
                "pdf_url": pdf_url,
                "fragility_triggers": found_fragilities,
                "snippet": snippet,
                "code_files": code_files
            })
            
    return candidates

def format_issue_markdown(candidate):
    trigger_words = ", ".join(candidate['fragility_triggers'])
    
    source_code_section = ""
    badge = ""
    if len(candidate['code_files']) > 0:
        badge = " [HAS SOURCE CODE]"
        files_bullet = "\n".join([f"- `{f}`" for f in candidate['code_files']])
        source_code_section = f"\n#### 📁 Source Code Included!\nThe authors have provided source files in their arXiv e-print bundle. This is a high-value candidate for immediate auditing.\n{files_bullet}\n"
    
    return f"""### [AUTO-AUDIT] arXiv:{candidate['id']}{badge} - {candidate['title']}

**🚨 Sanctuary Bot Alert:** Potential floating-point or integration fragility detected in recent preprint.
**Triggers:** `{trigger_words}`

- **Authors:** {candidate['authors']}
- **arXiv PDF:** {candidate['pdf_url']}
{source_code_section}
#### Abstract Snippet
> *"{candidate['snippet']}"*

#### Required Action (3BEP Community)
1. Analyze the paper and check if the initial conditions (masses, positions, velocities) are available or reproducible.
2. Translate the conditions into a `.bep` script format.
3. Run it on the 3BEP Sanctuary in I64F64 mode and activate the `--trajectory` flag.
4. Reply below with the verification results or mathematical divergence graphs.

*Let's build the Theory Graveyard.*
"""

def main():
    dry_run = "--dry-run" in sys.argv
    print(f"=== 3BEP Audit Bot Started {'(DRY RUN)' if dry_run else ''} ===")
    
    history = load_history()
    print(f"[*] Read {len(history['processed_ids'])} papers from history.")
    
    root = fetch_arxiv_papers(max_results=1000) # Fetch up to 1000 latest
    candidates = analyze_papers(root, history)
    
    print(f"[*] Found {len(candidates)} candidates for auditing.\n")
    
    for c in candidates:
        md_issue = format_issue_markdown(c)
        print("="*60)
        print(md_issue)
        print("="*60)
        
        if not dry_run:
            github_token = os.environ.get("GITHUB_TOKEN")
            repo = os.environ.get("GITHUB_REPOSITORY", "3beplabs/3bep-open-science-archive")
            
            if github_token:
                api_url = f"https://api.github.com/repos/{repo}/issues"
                data = json.dumps({
                    "title": f"[AUTO-AUDIT] arXiv:{c['id']} - Candidate for Verification",
                    "body": md_issue,
                    "labels": ["audit-candidate", "arxiv-bot"]
                }).encode("utf-8")
                
                req = urllib.request.Request(api_url, data=data, headers={
                    "Authorization": f"token {github_token}",
                    "Accept": "application/vnd.github.v3+json",
                    "Content-Type": "application/json"
                })
                
                try:
                    urllib.request.urlopen(req)
                    history["processed_ids"].append(c["id"])
                    print(f"[+] Issue created for: {c['id']}")
                except Exception as e:
                    print(f"[!] Error creating Issue for {c['id']}: {e}")
            else:
                # Local test fallback
                history["processed_ids"].append(c["id"])
                print(f"[!] Warning: No GITHUB_TOKEN; paper {c['id']} marked in history but Issue was not created.")
            
    if not dry_run and len(candidates) > 0:
        save_history(history)
        print("[*] History updated.")
        
    if len(candidates) == 0:
        print("[*] No new papers detected using the trigger keywords.")

if __name__ == "__main__":
    main()
