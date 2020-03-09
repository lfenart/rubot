const { spawn } = require('rubot_process');
const rubot = spawn('./rubot-holes', ['-c', 'holes.json']);

rubot.stdout.on('data', (data) => {
    console.log(`stdout: ${data}`);
});

rubot.stderr.on('data', (data) => {
    console.log(`stderr: ${data}`);
});

rubot.stdin.write("isready\n");
rubot.stdin.write("newgame IJLOSTZ\n");
rubot.stdin.write("isready\n");
rubot.stdin.write("go\n");
rubot.stdin.write("q\n");
rubot.stdin.write("exit\n");