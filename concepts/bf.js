var textareaInput = document.getElementById('input');
var buttonStart = document.getElementById('start');
var run = function (program) {
    var ram = [0];
    var callstack = [];
    var pc = 0;
    var sp = 0;
    var start = Date.now();
    while (program[pc] != 0 /* EOF */ && Date.now() - start < 5000) {
        switch (program[pc]) {
            case 1 /* INCREMENT */:
                ram[sp] < 255 ? ram[sp]++ : ram[sp] = 0;
                break;
            case 2 /* DECREMENT */:
                ram[sp] > 0 ? ram[sp]-- : ram[sp] = 255;
                break;
            case 3 /* LEFT */:
                sp--;
                typeof (ram[sp]) === 'number' ? null : ram[sp] = 0;
                break;
            case 4 /* RIGHT */:
                sp++;
                typeof (ram[sp]) === 'number' ? null : ram[sp] = 0;
                break;
            case 5 /* BEGIN */:
                callstack.push(pc - 1);
                break;
            case 6 /* END */:
                if (ram[sp] !== 0)
                    pc = callstack.pop();
                else
                    callstack.pop();
                break;
            case 7 /* OUTPUT */:
                console.log(String.fromCharCode(ram[sp]));
                break;
            case 8 /* INPUT */:
                ram[sp] = prompt().charCodeAt(0);
                break;
            case 0 /* EOF */:
                return;
        }
        pc++;
    }
};
var textToOps = function (text) {
    var ops = text.split('').map(function (char) {
        switch (char) {
            case '+': return 1 /* INCREMENT */;
            case '-': return 2 /* DECREMENT */;
            case '<': return 3 /* LEFT */;
            case '>': return 4 /* RIGHT */;
            case '[': return 5 /* BEGIN */;
            case ']': return 6 /* END */;
            case '.': return 7 /* OUTPUT */;
            case ',': return 8 /* INPUT */;
        }
    }).filter(function (v) { return typeof v === "number"; });
    ops.push(0 /* EOF */);
    return ops;
};
var main = function () {
    buttonStart.addEventListener('click', function () { return run(textToOps(textareaInput.value)); });
};
main();
