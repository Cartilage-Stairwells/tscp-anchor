const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const TSCP_ROOT = path.resolve(__dirname, '.');
const HARDHAT_TEST_CMD = 'npx hardhat test --network hardhat';

class GolfRunner {
  constructor() {
    this.results = [];
    this.startTime = Date.now();
  }

  log(phase, status, detail = '') {
    const ts = new Date().toISOString();
    const entry = `[${ts}] [GOLF] [${phase}] ${status} ${detail}`;
    console.log(entry);
    this.results.push(entry);
  }

  checkEcho() {
    this.log('ECHO', 'CHECK', 'Solidity contract...');
    const solPath = path.join(TSCP_ROOT, 'contracts', 'TSCPFriVerifier.sol');
    if (fs.existsSync(solPath)) {
      this.log('ECHO', 'PASS', `Contract: ${solPath}`);
      return true;
    }
    this.log('ECHO', 'FAIL', 'TSCPFriVerifier.sol missing');
    return false;
  }

  checkHardhat() {
    this.log('HARDHAT', 'CHECK', 'Config...');
    const cfgPath = path.join(TSCP_ROOT, 'hardhat.config.js');
    if (fs.existsSync(cfgPath)) {
      this.log('HARDHAT', 'PASS', `Config: ${cfgPath}`);
      return true;
    }
    this.log('HARDHAT', 'FAIL', 'hardhat.config.js missing');
    return false;
  }

  compileContracts() {
    this.log('GOLF', 'COMPILE', 'Running hardhat compile...');
    try {
      const out = execSync('npx hardhat compile', { cwd: TSCP_ROOT, encoding: 'utf-8', timeout: 120000 });
      this.log('GOLF', 'PASS', 'Compilation successful');
      return true;
    } catch (e) {
      this.log('GOLF', 'FAIL', `Compile error: ${e.message}`);
      return false;
    }
  }

  runTests() {
    this.log('GOLF', 'RUN', 'Executing Hardhat test suite...');
    try {
      const out = execSync(HARDHAT_TEST_CMD, { cwd: TSCP_ROOT, encoding: 'utf-8', timeout: 120000 });
      this.log('GOLF', 'PASS', 'All tests passed');
      console.log(out);
      return true;
    } catch (e) {
      this.log('GOLF', 'FAIL', `Test failure: ${e.message}`);
      console.error(e.stdout || '');
      console.error(e.stderr || '');
      return false;
    }
  }

  generateReport() {
    const duration = Date.now() - this.startTime;
    const report = {
      timestamp: new Date().toISOString(),
      duration_ms: duration,
      phases: this.results,
      status: this.results.some(r => r.includes('FAIL')) ? 'FAILED' : 'PASSED'
    };
    const p = path.join(TSCP_ROOT, 'golf_report.json');
    fs.writeFileSync(p, JSON.stringify(report, null, 2));
    this.log('GOLF', 'REPORT', `Saved to ${p}`);
    return report;
  }

  run() {
    console.log('\n╔══════════════════════════════════════════════════════════════════════════════╗');
    console.log('║  GOLF: Solidity Verifier Test & Deployment Pipeline                          ║');
    console.log('║  TSCP-PL v1.6.1-COLLOSSEUM :: Phase 9                                        ║');
    console.log('╚══════════════════════════════════════════════════════════════════════════════╝\n');

    const checks = [this.checkEcho(), this.checkHardhat()];
    if (!checks.every(c => c)) {
      this.log('GOLF', 'ABORT', 'Pre-checks failed');
      const r = this.generateReport();
      console.log(`\n  GOLF ${r.status}\n`);
      return false;
    }

    if (!this.compileContracts()) {
      const r = this.generateReport();
      console.log(`\n  GOLF ${r.status}\n`);
      return false;
    }

    this.runTests();
    const r = this.generateReport();
    console.log(`\n═══════════════════════════════════════════════════════════════════════════════`);
    console.log(`  GOLF ${r.status} in ${r.duration_ms}ms`);
    console.log(`═══════════════════════════════════════════════════════════════════════════════\n`);
    return r.status === 'PASSED';
  }
}

if (require.main === module) {
  const runner = new GolfRunner();
  const ok = runner.run();
  process.exit(ok ? 0 : 1);
}

module.exports = { GolfRunner };
