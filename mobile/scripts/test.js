#!/usr/bin/env node
/* G.A.N.E Node test runner */
const fs = require('fs');
const path = require('path');
const BUNDLE = path.join(__dirname, '..', 'www', 'gane-core-bundle.js');
if (!fs.existsSync(BUNDLE)) { console.error('Bundle not found:', BUNDLE); process.exit(1); }

global.window = {};
global.performance = { now: () => Date.now() };
global.navigator = { onLine: true, connection: { effectiveType: '4g' } };
global.localStorage = { _d:{}, setItem(k,v){this._d[k]=v}, getItem(k){return this._d[k]||null}, removeItem(k){delete this._d[k]}, get length(){return Object.keys(this._d).length}, key(i){return Object.keys(this._d)[i]} };
global.crypto = { getRandomValues: a => { for(let i=0;i<a.length;i++) a[i]=Math.floor(Math.random()*256); return a; } };

eval(fs.readFileSync(BUNDLE, 'utf8'));
const G = global.window.GANE;

(async () => {
  let total=0, pass=0, fail=0;
  const suites = [
    ['Core', G.Core.runAcceptanceTests()],
    ['Resilience', G.Resilience.runResilienceTests()],
    ['Reality', G.Reality.runRealityTests()],
    ['Consciousness', G.Consciousness.runConsciousnessTests()]
  ];
  console.log('\n═══ G.A.N.E TEST RUNNER ═══\n');
  for (const [name, r] of suites) {
    const p = r.filter(x=>x.includes('✓')).length;
    total += r.length; pass += p; fail += r.length - p;
    console.log(name.padEnd(15)+':', p+'/'+r.length, p===r.length?'✓':'✗');
    r.filter(x=>x.includes('✗')).forEach(x=>console.log('  ',x));
  }
  const cr = await G.Completion.runCompletionTests();
  total += cr.results.length; pass += cr.pass; fail += cr.fail;
  console.log('Completion     :', cr.pass+'/'+cr.results.length, cr.pass===cr.results.length?'✓':'✗');
  console.log('\n═══ '+pass+'/'+total+' tests passed ═══');
  process.exit(fail > 0 ? 1 : 0);
})();
