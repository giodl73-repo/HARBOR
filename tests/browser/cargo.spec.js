import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
async function ready(page){await page.goto('/HARBOR/#container-source');await expect(page.locator('#cargo-status')).toContainText('Source reconciliation ready');await expect(page.locator('#status')).toContainText('Comparison ready');}
async function download(page){const [file]=await Promise.all([page.waitForEvent('download'),page.locator('#cargo-download').click()]);return JSON.parse(readFileSync(await file.path(),'utf8'));}

test('source WASM matches four native canonical queries and preserves publisher discrepancy',async({page})=>{
 const requests=[];page.on('request',r=>requests.push(r.url()));await ready(page);
 const refs=JSON.parse(readFileSync('.cache/cargo-native-results.json','utf8'));
 const actual=await page.evaluate(async refs=>new Promise((resolve,reject)=>{
  const worker=new Worker('./worker.js?task=cargo',{type:'module'}),output=[];
  const timer=setTimeout(()=>{worker.terminate();reject(Error('source parity timeout'));},10000);
  function fail(error){clearTimeout(timer);worker.terminate();reject(error);}
  worker.onerror=()=>fail(Error('source parity worker error'));
  worker.onmessage=({data})=>{if(data.type==='ready')worker.postMessage({type:'cargo',id:0,input:refs[0].input});else if(data.ok){output.push(data.canonical);if(output.length===refs.length){clearTimeout(timer);worker.terminate();resolve(output);}else worker.postMessage({type:'cargo',id:output.length,input:refs[output.length].input});}else fail(Error(data.error));};worker.postMessage({type:'init'});
 }),refs);
 expect(actual).toEqual(refs.map(ref=>ref.canonical));await expect(page.locator('#cargo-rows tr')).toHaveCount(12);await expect(page.locator('#cargo-reconciliation tr')).toHaveCount(7);
 await expect(page.locator('#cargo-comparison')).toContainText('8,629,681.10');await expect(page.locator('#cargo-comparison')).toContainText('8,634,497.00');await expect(page.locator('#cargo-difference')).toContainText('-4,815.90');await expect(page.locator('#cargo-rounding')).toContainText('8.6');
 expect(requests.some(url=>url.endsWith('.html.txt'))).toBe(false);
});

test('month and loaded basis queries clear stale output and retain original total denominator',async({page})=>{
 await ready(page);await page.locator('#cargo-month').selectOption('12');await page.locator('#cargo-basis').selectOption('loaded');await expect(page.locator('#cargo-results')).toBeHidden();await expect(page.locator('#cargo-download')).toBeDisabled();await page.locator('#cargo-apply').click();await expect(page.locator('#cargo-status')).toContainText('Source reconciliation ready');
 await expect(page.locator('#cargo-rows tr')).toHaveCount(1);await expect(page.locator('#cargo-selection')).toContainText('486,235.75');
 const result=await download(page);expect(result.selection).toEqual({month:12,basis:'loaded',empty:false,sum_centi_teu:48623575});expect(result.rows[0].centi_teu[6]).toBe(74251900);expect(result.annual.total_centi_teu).toBe(862968110);expect(result.annual.loaded_centi_teu).toBe(573332700);expect(result.annual.empty_centi_teu).toBe(289635410);
 expect(result.annual.sum_minus_reported_centi_teu).toEqual([0,0,0,0,0,0,0]);expect(result.rows[0].component_residuals_centi_teu).toEqual([0,0,0]);expect(result.partial_fiscal_year.included_in_calendar_sums).toBe(false);expect(result.partial_fiscal_year.centi_teu[6]).toBe(449230185);
 expect(Object.values(result.observations).every(v=>v===null)).toBe(true);expect(Object.values(result.authority).every(v=>v===false)).toBe(true);expect(result.publisher_comparison.revision_chronology).toBeNull();expect(result.publisher_comparison.discrepancy_cause).toBeNull();expect(result.publisher_comparison.historical_corpus_original_source_recovered).toBe(false);
});

test('no month is absent not zero and reset retains independent hypothetical model',async({page})=>{
 await ready(page);await page.locator('#cargo-month').selectOption('none');await page.locator('#cargo-apply').click();await expect(page.locator('#cargo-status')).toContainText('Source reconciliation ready');await expect(page.locator('#cargo-rows tr')).toHaveCount(0);await expect(page.locator('#cargo-selection')).toContainText('absent, not zero');const empty=await download(page);expect(empty.selection.sum_centi_teu).toBeNull();expect(empty.annual.total_centi_teu).toBe(862968110);
 await page.locator('#clear').click();await expect(page.locator('#status')).toContainText('Comparison ready');await expect(page.locator('#coverage')).toContainText('12 dimensions remain unassessed');await expect(page.locator('#cargo-selection')).toContainText('No month selected');
 await page.locator('#cargo-reset').click();await expect(page.locator('#cargo-status')).toContainText('Source reconciliation ready');await expect(page.locator('#cargo-rows tr')).toHaveCount(12);await expect(page.locator('#metric-rows tr').first()).toContainText('No detected DIM-01 gap');
});

test('invalid and oversized source queries clear stale results and recover without changing scores',async({page})=>{
 await ready(page);await page.locator('#cargo-editor summary').click();
 for(const input of ['{"month":13,"basis":"total"}','{"month":0,"basis":"capacity"}','{"month":0,"basis":"total","depth":53}','not JSON','x'.repeat(65537)]){
  await page.locator('#cargo-input').fill(input);await expect(page.locator('#cargo-results')).toBeHidden();await expect(page.locator('#cargo-download')).toBeDisabled();await page.locator('#cargo-apply').click();await expect(page.locator('#cargo-status')).toContainText(input.length>65536?'exceeds 64 KiB':'Cannot inspect source query');
 }
 await expect(page.locator('#status')).toContainText('Comparison ready');await page.locator('#cargo-reset').click();await expect(page.locator('#cargo-status')).toContainText('Source reconciliation ready');
});

test('source worker loading and failure leave the model and static custody available',async({page})=>{
 let release;const blocked=new Promise(resolve=>{release=resolve;});await page.route('**/worker.js?task=cargo',async route=>{await blocked;await route.abort();});try{await page.goto('/HARBOR/#container-source');await expect(page.locator('#cargo-status')).toContainText('Loading source-reconciliation');}finally{release();}await expect(page.locator('#cargo-status')).toContainText('Unable to start source WebAssembly');await expect(page.locator('#cargo-results')).toBeHidden();await expect(page.locator('#status')).toContainText('Comparison ready');await expect(page.getByRole('link',{name:'Original table response (text)'})).toBeVisible();await expect(page.locator('#source-audit')).toContainText('Sixteen quantities needing primary sources');
});

test('keyboard mobile and exact publisher payload downloads work under the project prefix',async({page})=>{
 const errors=[];page.on('pageerror',error=>errors.push(error.message));await page.setViewportSize({width:390,height:844});await ready(page);await page.locator('#cargo-month').focus();await page.keyboard.press('Tab');await expect(page.locator('#cargo-basis')).toBeFocused();await page.keyboard.press('Tab');await expect(page.locator('#cargo-apply')).toBeFocused();await page.keyboard.press('Enter');await expect(page.locator('#cargo-status')).toContainText('Source reconciliation ready');expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
 const table=page.getByRole('region',{name:'Frozen monthly container quantities'});await table.focus();await page.keyboard.press('ArrowRight');await expect.poll(()=>table.evaluate(el=>el.scrollLeft)).toBeGreaterThan(0);
 const response=await page.request.get('/HARBOR/cargo/custody.json');expect(response.ok()).toBe(true);const custody=await response.json();expect(custody.table_publication_or_revision_date).toBeNull();expect(custody.news_publication_date).toBe('2024-01-16');
 for(const file of custody.files){const raw=await page.request.get('/HARBOR/cargo/'+file.file);expect(raw.status()).toBe(200);const bytes=await raw.body();expect(bytes.length).toBe(file.bytes);expect(createHash('sha256').update(bytes).digest('hex')).toBe(file.sha256);}
 expect((await page.request.get('/HARBOR/cargo/source.json')).ok()).toBe(true);expect((await page.request.get('/HARBOR/cargo/NOTICE.txt')).ok()).toBe(true);expect(errors).toEqual([]);
});
