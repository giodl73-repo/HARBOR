const $=id=>document.getElementById(id);
const format=new Intl.NumberFormat('en-US',{minimumFractionDigits:2,maximumFractionDigits:2});
const amount=centi=>format.format(centi/100);
const labels=['Loaded imports','Empty imports','Total imports','Loaded exports','Empty exports','Total exports','Total TEU'];
const initial='{"month":0,"basis":"total"}';
let worker,ready=false,revision=0,timer,result;
function node(tag,text){const element=document.createElement(tag);if(text!==undefined)element.textContent=text;return element;}
function row(label,values){const tr=node('tr'),th=node('th',label);th.scope='row';tr.append(th,...values.map(value=>node('td',value)));return tr;}
function status(text){$('cargo-status').textContent=text;}
function invalidate(text){revision++;clearTimeout(timer);result=null;$('cargo-results').hidden=true;$('cargo-download').disabled=true;if(text)status(text);}
function fail(text){ready=false;worker?.terminate();invalidate(text);for(const id of ['cargo-month','cargo-basis','cargo-apply','cargo-reset'])$(id).disabled=true;}
function render(data){
 const comparison=data.publisher_comparison;
 $('cargo-comparison').replaceChildren(row('Calendar-year total TEU',[amount(comparison.table_annual_centi_teu),amount(comparison.news_annual_centi_teu)]),row('December total TEU',[amount(comparison.table_december_centi_teu),amount(comparison.news_december_centi_teu)]),row('Calendar total, million TEU rounded to one decimal',[comparison.table_total_one_decimal_million,comparison.news_total_one_decimal_million]));
 $('cargo-difference').textContent=`Table minus news: ${amount(comparison.table_minus_news_annual_centi_teu)} TEU for calendar 2023; ${amount(comparison.table_minus_news_december_centi_teu)} TEU for December. Cause and revision chronology: unobserved.`;
 $('cargo-rounding').textContent=`Both publisher totals round to the retained ${comparison.retained_corpus_context_million}-million-TEU context: ${comparison.both_round_to_corpus_context?'yes':'no'}. Rounded agreement does not establish an exact historical value.`;
 $('cargo-selection').textContent=data.selection.empty?'No month selected. The selected subtotal is absent, not zero; annual source context is retained.':`${data.rows.length} selected month${data.rows.length===1?'':'s'} · ${data.selection.basis==='loaded'?'loaded TEU only':'total TEU, loaded + empty'}: ${amount(data.selection.sum_centi_teu)} TEU. All source columns retain their original denominator.`;
 $('cargo-rows').replaceChildren(...data.rows.map(month=>row(month.name,[...month.centi_teu.map(amount),amount(month.selected_basis_centi_teu)])));
 $('cargo-reconciliation').replaceChildren(...labels.map((label,index)=>row(label,[amount(data.annual.reported_centi_teu[index]),amount(data.annual.sum_of_months_centi_teu[index]),amount(data.annual.sum_minus_reported_centi_teu[index])])));
 $('cargo-components').textContent=`Annual loaded TEU: ${amount(data.annual.loaded_centi_teu)}; empty TEU: ${amount(data.annual.empty_centi_teu)}; total TEU: ${amount(data.annual.total_centi_teu)}. Annual component residuals (imports, exports, total): ${data.annual.component_residuals_centi_teu.map(amount).join(', ')} TEU. Monthly component residuals remain in the download.`;
 $('cargo-fiscal').textContent=`Separate source row: ${data.partial_fiscal_year.label}, ${amount(data.partial_fiscal_year.centi_teu[6])} total TEU. This partial fiscal window is excluded from calendar-year sums and comparisons.`;
 $('cargo-results').hidden=false;$('cargo-download').disabled=false;
}
function inspect(){
 if(!ready)return;invalidate('Reconciling frozen sources in Rust WebAssembly…');const input=$('cargo-input').value;
 if(new TextEncoder().encode(input).length>65536){status('Source query exceeds 64 KiB. Shorten the query.');return;}
 timer=setTimeout(()=>fail('Source calculation timed out. Reload to restart the worker.'),10000);worker.postMessage({type:'cargo',id:revision,input});
}
function select(){const value=$('cargo-month').value;$('cargo-input').value=JSON.stringify({month:value==='none'?null:Number(value),basis:$('cargo-basis').value});invalidate('Source query changed. Inspect again to refresh results.');}
for(const id of ['cargo-month','cargo-basis'])$(id).addEventListener('change',select);
$('cargo-input').addEventListener('input',()=>invalidate('Source query changed. Inspect again to refresh results.'));
$('cargo-apply').addEventListener('click',inspect);
$('cargo-reset').addEventListener('click',()=>{$('cargo-month').value='0';$('cargo-basis').value='total';$('cargo-input').value=initial;inspect();});
$('cargo-download').addEventListener('click',()=>{if(!result)return;const url=URL.createObjectURL(new Blob([JSON.stringify(result,null,2)],{type:'application/json'})),anchor=node('a');anchor.href=url;anchor.download='harbor-port-la-2023-source-results.json';anchor.click();setTimeout(()=>URL.revokeObjectURL(url),1000);});
try{
 const url=new URL('worker.js',import.meta.url);url.searchParams.set('task','cargo');worker=new Worker(url,{type:'module'});
 timer=setTimeout(()=>fail('Source WebAssembly loading timed out. Reload to retry.'),15000);worker.onerror=()=>fail('Unable to start source WebAssembly. Reload to retry.');
 worker.onmessage=({data})=>{
  if(data.type==='ready'){clearTimeout(timer);ready=true;for(const id of ['cargo-month','cargo-basis','cargo-apply','cargo-reset'])$(id).disabled=false;inspect();return;}
  if(data.type==='error'&&data.id===undefined){fail(`Unable to load source WebAssembly: ${data.error}`);return;}
  if(data.id!==revision)return;clearTimeout(timer);if(data.type==='error'||!data.ok){invalidate(`Cannot inspect source query: ${data.error}`);return;}
  result=data.result;render(result);status('Source reconciliation ready · computed locally in Rust WebAssembly.');
 };
 worker.postMessage({type:'init'});
}catch(error){fail(`Unable to start source WebAssembly: ${error.message}`);}
