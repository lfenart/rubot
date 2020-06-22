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

// sets a new game, first blocks are I, J, L, O, S, T, Z
rubot.stdin.write("newgame IJLOSTZ\n");

// reply: "readyok"
rubot.stdin.write("isready\n");

// start searching and returns the best move
rubot.stdin.write("go\n");

// play move hold = false, rotation = 1, translation = 2, spin = 3
rubot.stdin.write("play 0 1 2 3\n");

// add handicap line with a hole (or bomb) in column 0
rubot.stdin.write("handicap 0\n");

// add handicap lines with a hold in columns 1 (top) and 2 (bottom)
rubot.stdin.write("handicap 1 2\n");

// add block I
rubot.stdin.write("block I\n");

// add blocks O and S
rubot.stdin.write("block O S\n");

// ko, removes all handicap lines
rubot.stdin.write("ko\n");

// print the state of the robot
rubot.stdin.write("print\n");

// exit the processus
rubot.stdin.write("exit\n");
