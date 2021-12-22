
const textareaInput = document.getElementById('input') as HTMLTextAreaElement;
const buttonStart = document.getElementById('start') as HTMLButtonElement;

const enum Ops {
    EOF,
    INCREMENT,
    DECREMENT,
    LEFT,
    RIGHT,
    BEGIN,
    END,
    OUTPUT,
    INPUT,
}

const run = (program: Ops[]) => {
    const ram: number[] = [0];
    const callstack: number[] = [];
    let pc = 0;
    let sp = 0;
    const start = Date.now();
    while (program[pc] != Ops.EOF && Date.now() - start < 5000) {
        switch (program[pc]) {
            case Ops.INCREMENT:
                ram[sp] < 255 ? ram[sp]++ : ram[sp] = 0;
                break;
            case Ops.DECREMENT:
                ram[sp] > 0 ? ram[sp]-- : ram[sp] = 255;
                break;
            case Ops.LEFT:
                sp--;
                typeof(ram[sp]) === 'number' ? null : ram[sp] = 0;
                break;
            case Ops.RIGHT:
                sp++;
                typeof(ram[sp]) === 'number' ? null : ram[sp] = 0;
                break;
            case Ops.BEGIN:
                callstack.push(pc-1);
                break;
            case Ops.END:
                if (ram[sp] !== 0)
                    pc = callstack.pop()
                else
                    callstack.pop();
                
                break;
            case Ops.OUTPUT:
                console.log(String.fromCharCode(ram[sp]));
                break;
            case Ops.INPUT:
                ram[sp] = prompt().charCodeAt(0);
                break;
            case Ops.EOF:
                return;
        }
        pc++;
    }
}

const textToOps = (text: string): Ops[] => {
    const ops = text.split('').map<Ops>((char) => {
        switch (char) {
            case '+': return Ops.INCREMENT;
            case '-': return Ops.DECREMENT;
            case '<': return Ops.LEFT;
            case '>': return Ops.RIGHT;
            case '[': return Ops.BEGIN;
            case ']': return Ops.END;
            case '.': return Ops.OUTPUT;
            case ',': return Ops.INPUT;
        }
    }).filter(v => typeof v === "number");
    ops.push(Ops.EOF);
    return ops;
}

const main = () => {
    buttonStart.addEventListener('click', () => run(textToOps(textareaInput.value)));
}




main();