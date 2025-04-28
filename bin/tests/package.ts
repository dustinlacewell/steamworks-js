import { spawn } from 'child_process';
import which from 'which';

async function findBinary(command: string) {
  try {
    const resolvedPath = await which(command);
    if (!resolvedPath) {
      throw new Error(`Command not found: ${command}`);
    }
    return resolvedPath;
  } catch (err) {
    console.error(`Command not found: ${err}`);
    throw new Error(`Command not found: ${command}`);
  }
}

async function main() {
  process.chdir('bin/tests/app');
  const npx = await findBinary('npx');
  const child = spawn(npx, ['tsx', 'src/main.ts'], { stdio: 'inherit', shell: true });
  child.on('close', (code) => {
    if (code !== 0) {
      console.error(`Child process exited with code ${code}`);
      process.exit(1);
    }
    process.exit(0);
  });
  child.on('error', (err) => {
    console.error(`Failed to spawn child process: ${err.message}`);
    process.exit(1);
  });
}

main().catch(console.error);
