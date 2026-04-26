type Some<T> = {
    type: 'some';
    value: T;
};

type None = {
    type: 'none';
};

type Option<T> = Some<T> | None;

function some<T>(value: T): Some<T> {
    return { type: 'some', value };
}

function none(): None {
    return { type: 'none' };
}

function unwrap_or<T>(opt: Option<T>, defaultValue: T): T {
    if (opt.type === 'some') {
        return opt.value;
    } else {
        return defaultValue;
    }
}

console.log(unwrap_or(some(42), 0));
console.log(unwrap_or(none(), 0));
console.log(unwrap_or(some(-1), 99));
console.log(unwrap_or(none(), -1));