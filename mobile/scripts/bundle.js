#!/usr/bin/env node
/* Re-bundles compiled TS into single IIFE bundle */
const fs = require('fs');
const path = require('path');
const BUILD = path.join(__dirname, '..', 'build');
const OUT = path.join(__dirname, '..', 'www', 'gane-core-bundle.js');
const files = ['nav-engine-deep-core','top1-competitive-layer','resilience-layer','completion-pack','reality-loop','consciousness-layer']
  .map(n => path.join(BUILD, n+'.js'));
let bundle = `/* G.A.N.E Bundle ${new Date().toISOString()} */\n(function(global){\n'use strict';\n`;
for (const f of files) {
  if (!fs.existsSync(f)) { console.error('Missing:', f); process.exit(1); }
  let s = fs.readFileSync(f,'utf8')
    .replace(/^export\s+(class|const|function|async function|interface|type|enum)\s+/gm,'$1 ')
    .replace(/^export\s+\{[^}]*\};?\s*$/gm,'')
    .replace(/^import\s+.*?;?\s*$/gm,'');
  bundle += `\n/* ═══ ${path.basename(f)} ═══ */\n${s}\n`;
}
bundle += `\nglobal.GANE = {
  Core: typeof Core!=='undefined'?Core:{},
  Top1: typeof Top1Layer!=='undefined'?Top1Layer:{},
  Resilience: typeof Resilience!=='undefined'?Resilience:{},
  Completion: typeof Completion!=='undefined'?Completion:{},
  Reality: typeof RealityLoop!=='undefined'?RealityLoop:{},
  Consciousness: typeof Consciousness!=='undefined'?Consciousness:{}
};
})(typeof window!=='undefined'?window:globalThis);
`;
fs.writeFileSync(OUT, bundle);
console.log('Bundle written:', OUT, '('+bundle.length+' bytes)');
