const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const TSCP_ROOT = path.resolve(__dirname, '.');
const RUST_TEST_CMD = 'cargo test --lib foxtrot -- --nocapture';

class FoxtrotRunner {
    constructor() {
        this.results = [];
        this.startTime = Date.now();
    }

    log(phase, status, detail = '') {
        const timestamp = new Date().toISOString();
        const entry = `[${timestamp}] [FOXTROT] [${phase}] ${status} ${detail}`;
        console.log(entry);
        this.results.push(entry);
    }

    checkAlpha() {
        this.log('ALPHA', 'CHECK', 'Treasury snapshot...');
        const snapshotPath = path.join(TSCP_ROOT, 'treasury_snapshot.json');
        if (fs.existsSync(snapshotPath)) {
            this.log('ALPHA', 'PASS', `Snapshot found: ${snapshotPath}`);
            return true;
        }
        this.log('ALPHA', 'WARN', 'Snapshot not found, using synthetic data');
        return true;
    }

    checkBravo() {
        this.log('BRAVO', 'CHECK', 'HD key schema...');
        const keySchemaPath = path.join(TSCP_ROOT, 'TSCP-KEY-0001.json');
        if (fs.existsSync(keySchemaPath)) {
            this.log('BRAVO', 'PASS', `Key schema found: ${keySchemaPath}`);
            return true;
        }
        this.log('BRAVO', 'WARN', 'Key schema not found, using test keys');
        return true;
    }

    checkCharlie() {
        this.log('CHARLIE', 'CHECK', 'DEEP-ALI module...');
        const deepAliPath = path.join(TSCP_ROOT, 'src', 'deep_ali.rs');
        if (fs.existsSync(deepAliPath)) {
            this.log('CHARLIE', 'PASS', `Module found: ${deepAliPath}`);
            return true;
        }
        this.log('CHARLIE', 'FAIL', 'DEEP-ALI module missing');
        return false;
    }

    checkDelta() {
        this.log('DELTA', 'CHECK', 'FRI bridge module...');
        const deltaPath = path.join(TSCP_ROOT, 'src', 'delta_fri_bridge.rs');
        if (fs.existsSync(deltaPath)) {
            this.log('DELTA', 'PASS', `Module found: ${deltaPath}`);
            return true;
        }
        this.log('DELTA', 'FAIL', 'FRI bridge module missing');
        return false;
    }

    checkEcho() {
        this.log('ECHO', 'CHECK', 'Solidity verifier...');
        const solPath = path.join(TSCP_ROOT, 'contracts', 'TSCPFriVerifier.sol');
        if (fs.existsSync(solPath)) {
            const size = fs.statSync(solPath).size;
            this.log('ECHO', 'PASS', `Contract found: ${solPath} (${size} bytes)`);
            return true;
        }
        this.log('ECHO', 'FAIL', 'Solidity contract missing');
        return false;
    }

    runRustTests() {
        this.log('FOXTROT', 'RUN', 'Executing Rust integration tests...');
        try {
            const output = execSync(RUST_TEST_CMD, { cwd: TSCP_ROOT, encoding: 'utf-8', timeout: 120000 });
            this.log('FOXTROT', 'PASS', 'Rust tests completed');
            console.log(output);
            return true;
        } catch (error) {
            this.log('FOXTROT', 'FAIL', `Rust tests failed: ${error.message}`);
            console.error(error.stdout);
            console.error(error.stderr);
            return false;
        }
    }

    generateReport() {
        const duration = Date.now() - this.startTime;
        const report = { timestamp: new Date().toISOString(), duration_ms: duration, phases: this.results, status: this.results.some(r => r.includes('FAIL')) ? 'FAILED' : 'PASSED' };
        const reportPath = path.join(TSCP_ROOT, 'foxtrot_report.json');
        fs.writeFileSync(reportPath, JSON.stringify(report, null, 2));
        this.log('FOXTROT', 'REPORT', `Saved to ${reportPath}`);
        return report;
    }

    run() {
        console.log('\n╔══════════════════════════════════════════════════════════════════════════════╗');
        console.log('║  FOXTROT: End-to-End Integration Test Harness                              ║');
        console.log('║  TSCP-PL v1.6.1-COLLOSSEUM :: Phase 8                                       ║');
        console.log('╚══════════════════════════════════════════════════════════════════════════════╝\n');

        const checks = [this.checkAlpha(), this.checkBravo(), this.checkCharlie(), this.checkDelta(), this.checkEcho()];
        if (checks.every(c => c)) { this.runRustTests(); }
        else { this.log('FOXTROT', 'ABORT', 'Pre-checks failed, skipping Rust tests'); }

        const report = this.generateReport();
        console.log('\n═══════════════════════════════════════════════════════════════════════════════');
        console.log(`  FOXTROT ${report.status} in ${report.duration_ms}ms`);
        console.log('═══════════════════════════════════════════════════════════════════════════════\n');
        return report.status === 'PASSED';
    }
}

if (require.main === module) {
    const runner = new FoxtrotRunner();
    const success = runner.run();
    process.exit(success ? 0 : 1);
}

module.exports = { FoxtrotRunner };
