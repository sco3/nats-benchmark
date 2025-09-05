
import * as fs from 'fs';
import * as path from 'path';

interface LogData {
    [key: string]: {
        pub?: string;
        sub?: string;
    };
}

function parseLogFile(filePath: string): string | null {
    const content = fs.readFileSync(filePath, 'utf-8');
    const match = content.match(/(\d{1,3}(?:,\d{3})*|\d+)\s+msgs\/sec/);
    return match ? match[1].replace(/,/g, '') : null;
}

function getSortKey(dirName: string): number {
    dirName = dirName.toLowerCase();
    if (dirName.endsWith('ms')) {
        return parseInt(dirName.slice(0, -2), 10);
    } else if (dirName.endsWith('s')) {
        return parseInt(dirName.slice(0, -1), 10) * 1000;
    } else if (dirName.endsWith('m')) {
        return parseInt(dirName.slice(0, -1), 10) * 60 * 1000;
    } else {
        return Infinity; // Should not happen
    }
}

function main() {
    const data: LogData = {};
    const currentDir = './';

    const dirs = fs.readdirSync(currentDir).filter(f => fs.statSync(path.join(currentDir, f)).isDirectory());

    for (const dir of dirs) {
        const pubLogPath = path.join(currentDir, dir, 'bench-pub.log');
        const subLogPath = path.join(currentDir, dir, 'bench-sub.log');

        if (fs.existsSync(pubLogPath)) {
            const pubMsgsSec = parseLogFile(pubLogPath);
            if (pubMsgsSec) {
                if (!data[dir]) data[dir] = {};
                data[dir].pub = pubMsgsSec;
            }
        }

        if (fs.existsSync(subLogPath)) {
            const subMsgsSec = parseLogFile(subLogPath);
            if (subMsgsSec) {
                if (!data[dir]) data[dir] = {};
                data[dir].sub = subMsgsSec;
            }
        }
    }

    const sortedDirs = Object.keys(data).sort((a, b) => getSortKey(a) - getSortKey(b));

    console.log('| Sync Period | Pub r/s | Sub r/s |');
    console.log('|---|---|---|');

    for (const dirName of sortedDirs) {
        const pubRs = data[dirName].pub || 'N/A';
        const subRs = data[dirName].sub || 'N/A';
        console.log(`| ${dirName} | ${pubRs} | ${subRs} |`);
    }
}

main();
