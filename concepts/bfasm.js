var textareaInput = document.getElementById('input');
var buttonStart = document.getElementById('start');
var Operation;
(function (Operation) {
    Operation["INCREMENT"] = "INCREMENT";
    Operation["DECREMENT"] = "DECREMENT";
    Operation["LEFT"] = "LEFT";
    Operation["RIGHT"] = "RIGHT";
    Operation["JUMP"] = "JUMP";
    Operation["OUTPUT"] = "OUTPUT";
    Operation["INPUT"] = "INPUT";
})(Operation || (Operation = {}));
var run = function (program) {
    var ram = [0];
    var pc = 0;
    var sp = 0;
    var start = Date.now();
    while (pc < program.length && Date.now() - start < 2000) {
        switch (program[pc].operation) {
            case Operation.INCREMENT:
                ram[sp] < 255 ? ram[sp]++ : ram[sp] = 0;
                console.log(ram[sp]);
                break;
            case Operation.DECREMENT:
                ram[sp] > 0 ? ram[sp]-- : ram[sp] = 255;
                break;
            case Operation.LEFT:
                sp--;
                typeof (ram[sp]) === 'number' ? null : ram[sp] = 0;
                break;
            case Operation.RIGHT:
                sp++;
                typeof (ram[sp]) === 'number' ? null : ram[sp] = 0;
                break;
            case Operation.JUMP:
                if (ram[sp] !== 0) {
                    pc = program[pc].value - 1;
                }
                break;
            case Operation.OUTPUT:
                console.log(String.fromCharCode(ram[sp]));
                break;
            case Operation.INPUT:
                ram[sp] = prompt().charCodeAt(0);
                break;
        }
        pc++;
    }
};
var textToOps = function (text) {
    var program = [];
    var brackets = [];
    var pc = 0;
    for (var _i = 0, _a = text.split(''); _i < _a.length; _i++) {
        var char = _a[_i];
        switch (char) {
            case '+':
                program.push({ operation: Operation.INCREMENT, value: 0 });
                break;
            case '-':
                program.push({ operation: Operation.DECREMENT, value: 0 });
                break;
            case '<':
                program.push({ operation: Operation.LEFT, value: 0 });
                break;
            case '>':
                program.push({ operation: Operation.RIGHT, value: 0 });
                break;
            case '[':
                brackets.push(pc);
                pc--;
                break;
            case ']':
                program.push({ operation: Operation.JUMP, value: brackets.pop() });
                break;
            case '.':
                program.push({ operation: Operation.OUTPUT, value: 0 });
                break;
            case ',':
                program.push({ operation: Operation.INPUT, value: 0 });
                break;
            default:
                pc--;
        }
        pc++;
    }
    return program;
};
var main = function () {
    buttonStart.addEventListener('click', function () { return run(textToOps(textareaInput.value)); });
};
main();
