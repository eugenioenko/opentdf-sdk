// Full-mode runner of the consumer's offline controlled checks (see
// controlled-negatives.mjs); the focused jobs run them through run-focused.py.
import {resolve} from 'node:path';
import {mkdir} from 'node:fs/promises';
import {spawnSync} from 'node:child_process';
const sdk=resolve(import.meta.dirname,'../../..'),run=resolve(sdk,'.local/typescript-tdf-library/controlled');
await mkdir(run,{recursive:true});
const result=spawnSync('node',[resolve(import.meta.dirname,'main.mjs'),sdk,run,'controlled','all'],{stdio:'inherit'});
process.exit(result.status??1);
