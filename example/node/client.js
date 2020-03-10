const { spawn } = require('child_process');

// modify this to match your paths
const path = './rubot-holes';
const config = 'holes.json';

const rubot = spawn(path, ['-c', config]);

rubot.stdout.on('data', (data) => {
    console.log(`stdout: ${data}`);
});

rubot.stderr.on('data', (data) => {
    console.log(`stderr: ${data}`);
});

// reply: "nok"
rubot.stdin.write("isready\n");

// sets a new game, first blocks are I, J, L, O, S, T, Z
rubot.stdin.write("newgame IJLOSTZ\n");

// reply: "readyok"
rubot.stdin.write("isready\n");

// start the game
rubot.stdin.write("go\n");

// no ko, next block is I, 3 handicap lines added on columns 3, 4 and 5
rubot.stdin.write("0 I 3 4 5\n");

// ko, next block is O, no handicap line added
rubot.stdin.write("1 O\n");

// quit the game
rubot.stdin.write("q\n");

// exit the processus
rubot.stdin.write("exit\n");
