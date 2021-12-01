import { read, readFileSync } from 'fs';

const windowed = (array) => array.reduce((acc, item, index) => [...acc, array.slice(index, index + 3)], []);
const sumArray = array => array.reduce((a, b) => a + b);

const one = (arr) =>
    arr.reduce(
        (acc, current, index, arr) =>
            current > arr[index - 1] ? acc + 1 : acc,
        0,
    );

const two = (arr) => one(windowed(arr).map(sumArray))

// .

const data = readFileSync('./input1.txt', 'utf-8').split('\n').map(Number);

console.log(one(data));

console.log(two(data));
