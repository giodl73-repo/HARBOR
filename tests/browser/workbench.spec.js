import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
async function ready(page){await page.goto('/HARBOR/');await expect(page.getByRole('status')).toContainText('Comparison ready');}

test('executing WASM matches native canonical bytes for reset, systemic and missing inputs',async({page})=>{
 await ready(page);const refs=JSON.parse(readFileSync('.cache/native-results.json','utf8'));
 const actual=await page.evaluate(async refs=>new Promise((resolve,reject)=>{const worker=new Worker('./worker.js',{type:'module'}),output=[];const timer=setTimeout(()=>{worker.terminate();reject(Error('parity timeout'));},10000);worker.onerror=()=>{clearTimeout(timer);worker.terminate();reject(Error('parity worker error'));};worker.onmessage=({data})=>{if(data.type==='ready')worker.postMessage({type:'evaluate',id:0,input:refs[0].input});else if(data.ok){output.push(data.canonical);if(output.length===refs.length){clearTimeout(timer);worker.terminate();resolve(output);}else worker.postMessage({type:'evaluate',id:output.length,input:refs[output.length].input});}else{clearTimeout(timer);worker.terminate();reject(Error(data.error));}};worker.postMessage({type:'init'});}),refs);
 expect(actual).toEqual(refs.map(ref=>ref.canonical));await expect(page.locator('#score-controls input')).toHaveCount(8);
 const minimum=page.locator('.detector[data-source="UnderServedRegion"]');const tail=page.locator('.detector[data-source="TailRegion"]');await expect(minimum.locator('li')).toHaveCount(8);await expect(tail.locator('li')).toHaveCount(2);await expect(tail).toContainText('Houston');await expect(tail).toContainText('Savannah');
});

test('hypothetical systemic and cleared scores retain twelve coverage gaps',async({page})=>{
 await ready(page);await page.getByRole('button',{name:'Try systemic pattern'}).click();await expect(page.getByRole('status')).toContainText('Comparison ready');await expect(page.locator('#metric-rows tr').first()).toContainText('Systemic pattern');await expect(page.getByRole('heading',{name:'Systemic pattern · named entries',exact:true})).toBeVisible();
 await page.getByRole('button',{name:'Clear assessed deficits'}).click();await expect(page.getByRole('status')).toContainText('Comparison ready');await expect(page.locator('#metric-rows tr').first()).toContainText('No detected DIM-01 gap');await expect(page.locator('#coverage')).toContainText('12 dimensions remain unassessed');await expect(page.locator('#overall')).toContainText('unavailable');
 await page.getByRole('button',{name:'Reset illustration'}).click();await expect(page.getByRole('status')).toContainText('Comparison ready');await expect(page.locator('#metric-rows tr').first()).toContainText('Concentrated tail');
});

test('a missing score is absent evidence in native results and downloads',async({page})=>{
 await ready(page);await page.getByRole('spinbutton').first().fill('');await page.getByRole('spinbutton').first().press('Tab');await expect(page.getByRole('status')).toContainText('Comparison ready');await expect(page.locator('#metric-rows tr').nth(1).locator('td').last()).toHaveText('7');await expect(page.locator('#score-controls')).toContainText('Not observed');
 const [download]=await Promise.all([page.waitForEvent('download'),page.getByRole('button',{name:'Download results JSON'}).click()]);const result=JSON.parse(readFileSync(await download.path(),'utf8'));expect(result.hypothetical).toBe(true);expect(result.current.entries[0].score).toBeNull();expect(result.current.scored_entries).toBe(7);expect(result.current.overall_null).toBeNull();expect(result.current.authority.navigation_clearance).toBe(false);
});

test('all missing scores report thirteen unassessed dimensions, not zero performance',async({page})=>{
 await ready(page);await page.locator('#editor summary').click();const input=JSON.parse(await page.locator('#input').inputValue());for(const entry of input.entries)entry.score=null;await page.locator('#input').fill(JSON.stringify(input));await page.getByRole('button',{name:'Compare scores'}).click();await expect(page.getByRole('status')).toContainText('Comparison ready');await expect(page.locator('#metric-rows tr').first()).toContainText('Not assessed');await expect(page.locator('#coverage')).toContainText('13 dimensions remain unassessed');await expect(page.locator('#score-controls .score-fill')).toHaveCount(0);
});

test('invalid scale scores and oversized inputs clear stale results and recover',async({page})=>{
 await ready(page);await page.locator('#editor summary').click();const original=await page.locator('#input').inputValue();const outOfRange=JSON.parse(original);outOfRange.entries[0].score=11;
 for(const input of [original.replace('national','local'),JSON.stringify(outOfRange),'{"person_name":"example"}','x'.repeat(65537)]){await page.locator('#input').fill(input);await expect(page.locator('#results')).toBeHidden();await expect(page.locator('#download')).toBeDisabled();await page.getByRole('button',{name:'Compare scores'}).click();await expect(page.getByRole('status')).toContainText(input.length>65536?'exceeds 64 KiB':'Cannot compare');}
 await page.getByRole('button',{name:'Reset illustration'}).click();await expect(page.getByRole('status')).toContainText('Comparison ready');
});

test('loading and worker failure remain explicit',async({page})=>{
 await page.route('**/*.wasm',async route=>{await new Promise(resolve=>setTimeout(resolve,400));await route.abort();});await page.goto('/HARBOR/');await expect(page.getByRole('status')).toContainText('Loading Rust');await expect(page.getByRole('status')).toContainText('Unable to load Rust WebAssembly');await expect(page.locator('#results')).toBeHidden();
});

test('keyboard mobile and exact source-custody artifacts work under HARBOR prefix',async({page})=>{
 const errors=[];page.on('pageerror',error=>errors.push(error.message));await page.setViewportSize({width:390,height:844});await ready(page);await page.locator('#compare').focus();await page.keyboard.press('Tab');await expect(page.locator('#reset')).toBeFocused();await page.keyboard.press('Enter');await expect(page.getByRole('status')).toContainText('Comparison ready');expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
 for(const path of ['pkg/harbor_web.wasm','example.json','systemic.json','missing.json','LICENSE','THIRD-PARTY-LICENSES.txt','provenance.json','source-custody.json'])expect((await page.request.get('/HARBOR/'+path)).ok()).toBe(true);
 const custody=await (await page.request.get('/HARBOR/source-custody.json')).json();expect(custody).toHaveLength(10);for(const file of custody)expect((await page.request.get('/HARBOR/'+file.published)).ok()).toBe(true);expect(errors).toEqual([]);
});
