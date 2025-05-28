import { bold, white } from 'picocolors';
export function finalNote(args) {
    const lines = [
        `That's it!`,
        `Change to your new directory and start developing:`,
        msg(`cd ${args.target}`),
        `Start the app:`,
        cmd(args.packageManager, 'dev'),
        ...args.instructions.map((line) => (line.startsWith('+') ? msg(line.slice(1)) : line)),
    ];
    return lines.join('\n\n');
}
function cmd(pm, command) {
    return msg(`${pm} run ${command}`);
}
function msg(message) {
    return bold(white(message));
}
