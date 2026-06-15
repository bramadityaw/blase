import { mkdirSync, copyFileSync, rmSync } from 'node:fs';
import * as path from 'node:path';

const EXT_DIR = './editors/vscode';
const EXT_BIN_DIR = path.join(EXT_DIR, 'dist');

const [_node, _script, target_bin, ..._] = process.argv;
try {
    rmSync(EXT_BIN_DIR, { recursive: true });
} catch (_) { }
mkdirSync(EXT_BIN_DIR, { recursive: true });

const bin_name = path.basename(target_bin);
copyFileSync(target_bin, path.join(EXT_BIN_DIR, bin_name));
