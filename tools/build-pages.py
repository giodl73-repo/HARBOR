"""Build a historical-model replay with exact source custody and native parity."""
import hashlib
import json
import re
import shutil
import subprocess
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]

def output(*args):return subprocess.check_output(args,cwd=ROOT,text=True,encoding='utf-8')

def main():
    reset=json.loads((ROOT/'fixtures/browser-reset.json').read_text())
    source_rows={}
    for file in sorted((ROOT/'corpus').glob('us-*.md')):
        text=file.read_text(encoding='utf-8')
        identity=re.search(r'^id:\s*(.+)$',text,re.M).group(1).strip()
        score=float(re.search(r'^score: DIM-01\s*\|\s*([0-9.]+)',text,re.M).group(1))
        tier=re.search(r'^tier:\s*(.+)$',text,re.M).group(1).strip()
        source_rows[identity]=(score,tier)
    if len(source_rows)!=8 or {row['id']:(row['score'],row['tier']) for row in reset['entries']}!=source_rows:raise ValueError('Reset fixture drifted from the committed repository corpus')
    cache=ROOT/'.cache';cache.mkdir(exist_ok=True);refs=[]
    for mode in ['reset','systemic','missing']:
        path=f'fixtures/browser-{mode}.json';refs.append({'input':(ROOT/path).read_text(),'canonical':output('cargo','run','--locked','--quiet','-p','harbor-web','--bin','harbor-browser','--',path).strip()})
    (cache/'native-results.json').write_text(json.dumps(refs),encoding='utf-8')
    subprocess.run(['cargo','build','--locked','--release','--target','wasm32-unknown-unknown','-p','harbor-web','--lib'],cwd=ROOT,check=True)
    metadata=json.loads(output('cargo','metadata','--no-deps','--format-version','1'))
    dest=ROOT/'dist/HARBOR';(dest/'pkg').mkdir(parents=True,exist_ok=True)
    shutil.copy2(Path(metadata['target_directory'])/'wasm32-unknown-unknown/release/harbor_web.wasm',dest/'pkg/harbor_web.wasm')
    for file in (ROOT/'web').iterdir():
        if file.is_file():shutil.copy2(file,dest/file.name)
    for mode,target in [('reset','example.json'),('systemic','systemic.json'),('missing','missing.json')]:shutil.copy2(ROOT/f'fixtures/browser-{mode}.json',dest/target)
    for name in ['LICENSE','THIRD-PARTY-LICENSES.txt']:shutil.copy2(ROOT/name,dest/name)
    (dest/'evidence').mkdir(exist_ok=True)
    sources=[(file,f'evidence/{file.name}') for file in sorted((ROOT/'corpus').glob('us-*.md'))]
    sources.extend([(ROOT/'docs/findings/2026-06-deep-draft-channel-tail.md','evidence/finding.md'),(ROOT/'data/sources.md','evidence/sources.md')])
    custody=[]
    for file,target in sources:
        shutil.copy2(file,dest/target);custody.append({'source':file.relative_to(ROOT).as_posix(),'published':target,'sha256':hashlib.sha256(file.read_bytes()).hexdigest()})
    (dest/'source-custody.json').write_text(json.dumps(custody,indent=2),encoding='utf-8')
    provenance={'commit':output('git','rev-parse','HEAD').strip(),'dirty':bool(output('git','status','--porcelain').strip()),'rust':output('rustc','--version').strip(),'source':'https://github.com/giodl73-repo/HARBOR','scope':'hypothetical replay of June 2026 repository illustration; 2023 source context'}
    (dest/'provenance.json').write_text(json.dumps(provenance,indent=2),encoding='utf-8');(dest/'.nojekyll').touch()
    size=sum(file.stat().st_size for file in dest.rglob('*') if file.is_file())
    if size>=5_000_000:raise ValueError('Artifact exceeds 5 MB')
    print(f'HARBOR Pages: {size:,} bytes; source fixture matches all eight committed gateways')

if __name__=='__main__':main()
