"""Build a historical-model replay with exact source custody and native parity."""
import hashlib
import html
import json
import re
import shutil
import subprocess
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]

ORIGINAL_FINDING_SHA256='93f59553afd315aba5ca610794311b95444a015240212aebfc89cd86e4103f72'

def source_audit():
    original=ROOT/'data/history/2026-06-deep-draft-channel-tail-original.md'
    if hashlib.sha256(original.read_bytes()).hexdigest()!=ORIGINAL_FINDING_SHA256:
        raise ValueError('Historical original finding bytes changed')
    rows=[]
    for file in sorted((ROOT/'corpus').glob('us-*.md')):
        text=file.read_text(encoding='utf-8')
        identity=re.search(r'^id:\s*(.+)$',text,re.M).group(1).strip()
        name=re.search(r'^# (.+)$',text,re.M).group(1).strip()
        score=float(re.search(r'^score: DIM-01\s*\|\s*([0-9.]+)',text,re.M).group(1))
        quantities=[]
        for line in text.splitlines():
            if not line.startswith('quantity:'):continue
            value,unit,label,source,basis=[part.strip() for part in line.removeprefix('quantity:').split('|')]
            if label!='source-needed':raise ValueError('Unverified quantity must be source-needed')
            quantities.append({'value':float(value),'unit':unit,'evidence_label':label,'source_id':source,'demand_basis':basis,'primary_source_verified':False,'primary_source_locator':None,'primary_source_sha256':None})
        expected={('feet-channel-depth-mlw','usace-channel-depth-2023'),('million-teu-2023','port-teu-2023')}
        if len(quantities)!=2 or {(q['unit'],q['source_id']) for q in quantities}!=expected:
            raise ValueError('Expected one depth and one TEU quantity per gateway')
        rows.append({'id':identity,'name':name,'corpus_path':file.relative_to(ROOT).as_posix(),'published_corpus':'evidence/'+file.name,'corpus_sha256':hashlib.sha256(file.read_bytes()).hexdigest(),'illustrative_score':score,'quantities':quantities})
    if len(rows)!=8 or len({row['id'] for row in rows})!=8:raise ValueError('Expected eight unique gateways')
    return {'schema':'harbor.historical-source-custody-audit.v1','audit_date':'2026-10-26','scope':'national historical repository model; not verified physical observations','quantity_count':16,'verified_primary_quantities':0,'unverified_quantities':16,'original_finding':{'published':'evidence/original-finding.md','sha256':ORIGINAL_FINDING_SHA256,'status':'superseded; navigation conclusions withdrawn'},'gateways':rows}

def audit_table(audit):
    rows=[]
    for row in audit['gateways']:
        q={q['unit']:q['value'] for q in row['quantities']}
        rows.append('<tr><th scope="row"><a href="'+html.escape(row['published_corpus'],quote=True)+'">'+html.escape(row['name'])+'</a></th><td>'+str(q['feet-channel-depth-mlw'])+'</td><td>'+str(q['million-teu-2023'])+'</td><td>'+str(row['illustrative_score'])+'</td><td>Source needed for both quantities</td></tr>')
    return ''.join(rows)

def output(*args):return subprocess.check_output(args,cwd=ROOT,text=True,encoding='utf-8')

def main():
    audit=source_audit()
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
    (dest/'source-audit.json').write_bytes((json.dumps(audit,indent=2)+'\n').encode('utf-8'))
    for file in (ROOT/'web').iterdir():
        if file.is_file():shutil.copy2(file,dest/file.name)
    index=(dest/'index.html').read_text(encoding='utf-8')
    if index.count('<!-- SOURCE_AUDIT_ROWS -->')!=1:raise ValueError('Source audit table marker missing or duplicated')
    (dest/'index.html').write_bytes(index.replace('<!-- SOURCE_AUDIT_ROWS -->',audit_table(audit)).encode('utf-8'))
    for mode,target in [('reset','example.json'),('systemic','systemic.json'),('missing','missing.json')]:shutil.copy2(ROOT/f'fixtures/browser-{mode}.json',dest/target)
    for name in ['LICENSE','THIRD-PARTY-LICENSES.txt']:shutil.copy2(ROOT/name,dest/name)
    (dest/'evidence').mkdir(exist_ok=True)
    sources=[(file,f'evidence/{file.name}') for file in sorted((ROOT/'corpus').glob('us-*.md'))]
    sources.extend([(ROOT/'docs/findings/2026-06-deep-draft-channel-tail.md','evidence/finding.md'),(ROOT/'data/sources.md','evidence/sources.md')])
    sources.extend([(ROOT/'data/source-audit.md','evidence/source-audit.md'),(ROOT/'data/history/2026-06-deep-draft-channel-tail-original.md','evidence/original-finding.md')])
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
