
const textareaInput = document.getElementById('input') as HTMLTextAreaElement;
const buttonStart = document.getElementById('start') as HTMLButtonElement;

enum Operation {
    INCREMENT = 'INCREMENT',
    DECREMENT = 'DECREMENT',
    LEFT = 'LEFT',
    RIGHT = 'RIGHT',
    JUMP = 'JUMP',
    OUTPUT = 'OUTPUT',
    INPUT = 'INPUT',
}

type Instruction = {
    operation: Operation,
    value: number
}

const run = (program: Instruction[]) => {
    const ram: number[] = [0];
    let pc = 0;
    let sp = 0;
    const start = Date.now();
    while (pc < program.length && Date.now() - start < 2000) {
        switch (program[pc].operation) {
            case Operation.INCREMENT:
                ram[sp] < 255 ? ram[sp]++ : ram[sp] = 0;
                console.log(ram[sp])
                break;
            case Operation.DECREMENT:
                ram[sp] > 0 ? ram[sp]-- : ram[sp] = 255;
                break;
            case Operation.LEFT:
                sp--;
                typeof(ram[sp]) === 'number' ? null : ram[sp] = 0;
                break;
            case Operation.RIGHT:
                sp++;
                typeof(ram[sp]) === 'number' ? null : ram[sp] = 0;
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
}

const textToOps = (text: string): Instruction[] => {
    const program: Instruction[] = [];
    const brackets: number[] = [];
    let pc = 0;
    for (let char of text.split('')) {
        switch (char) {
            case '+':
                program.push({operation: Operation.INCREMENT, value: 0});
                break;
            case '-':
                program.push({operation: Operation.DECREMENT, value: 0});
                break;
            case '<':
                program.push({operation: Operation.LEFT, value: 0});
                break;
            case '>':
                program.push({operation: Operation.RIGHT, value: 0});
                break;
            case '[':
                brackets.push(pc);
                pc--;
                break;
            case ']':
                program.push({operation: Operation.JUMP, value: brackets.pop()});
                break;
            case '.':
                program.push({operation: Operation.OUTPUT, value: 0});
                break;
            case ',':
                program.push({operation: Operation.INPUT, value: 0});
                break;
            default:
                pc--;
        }
        pc++;
    }
    return program;
}

const main = () => {
    buttonStart.addEventListener('click', () => run(textToOps(textareaInput.value)));
}




main();