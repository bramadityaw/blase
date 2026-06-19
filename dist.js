import * as fs from 'node:fs';
import * as proc from 'node:process';
import * as cproc from 'node:child_process';
import * as path from 'node:path';

const EXT_DIR = path.join('editors', 'vscode');
const EXT_BIN_DIR = path.join(EXT_DIR, 'dist');

const withDir = (dir, cb) => {
    const pwd = proc.cwd();
    process.chdir(dir);
    cb();
    process.chdir(pwd);
};

const runCmd = (cmd, ...args) => {
    const command = [cmd, ...args].join(' ');
    cproc.execSync(command, {
        stdio: 'inherit',
    });
}

let [_node, _script, rust_target, code_target, bin_name, tag, ..._] = process.argv;

tag = tag ?? (() => {
    const out = cproc.execSync('git describe', {
        encoding: 'utf8',
    }).toString().trim();
    return out;
})();

rust_target = rust_target ?? 'x86_64-pc-windows-msvc';
code_target = code_target ?? 'win32-x64';
if (!bin_name) {
    bin_name = 'blase'
    if (proc.platform == 'win32') {
        bin_name += '.exe'
    }
}

try {
    fs.rmSync(EXT_BIN_DIR, { recursive: true });
} catch (_) { }

fs.mkdirSync(EXT_BIN_DIR, { recursive: true });

runCmd(
    'cargo build',
    '--release',
    '--package blase',
    '--manifest-path ./crates/blase/Cargo.toml',
    '--target', rust_target,
);
const bin_dir = path.join('target', rust_target, 'release');
fs.copyFileSync(path.join(bin_dir, bin_name), path.join(EXT_BIN_DIR, bin_name));

withDir(EXT_DIR, () => {
    runCmd('npm ci');
    runCmd(
        'npx --no-install',
        'vsce package',
        '--target', code_target,
        '--out', `blase-${tag}-${code_target}.vsix`,
    );
});
